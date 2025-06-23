pub(super) fn set_rest(stream: &mut TokenStream) -> Result<SetRest> {

    /*
          SESSION CHARACTERISTICS AS TRANSACTION transaction_mode_list
        | SESSION AUTHORIZATION session_auth_user
        | TRANSACTION SNAPSHOT SCONST
        | TRANSACTION transaction_mode_list
        | set_rest_more
    */

    choice!(
        {
            (
                Session,
                choice!(
                    (Characteristics, As, Transaction, transaction_mode_list)
                        .map(|(.., modes)| SetRest::SessionTransactionCharacteristics(modes)),
                    (Authorization, session_auth_user)
                        .right()
                        .map(|user| SetRest::SessionAuthorization { user })
                )
            )
            .right::<_, SetRest>()
        },
        {
            (
                Transaction,
                choice!(
                    (Snapshot, string)
                        .right()
                        .map(SetRest::TransactionSnapshot),
                    transaction_mode_list
                        .map(SetRest::LocalTransactionCharacteristics)
                )
            )
            .right::<_, SetRest>()
        },
        set_rest_more
    )
        .parse(stream)
}

pub(super) fn set_rest_more(stream: &mut TokenStream) -> Result<SetRestMore> {

    /*
          SESSION AUTHORIZATION session_auth_user
        | TRANSACTION SNAPSHOT SCONST
        | TIME ZONE zone_value
        | CATALOG_P SCONST
        | SCHEMA SCONST
        | NAMES opt_encoding
        | ROLE NonReservedWord_or_Sconst
        | XML_P OPTION document_or_content
        | var_name FROM CURRENT_P
        | var_name generic_set_tail
    */

    // All keywords conflict with `var_name`, so it needs to be last

    choice!(
        (Session, Authorization, session_auth_user)
            .map(|(.., user)| SetRestMore::SessionAuthorization { user }),
        (Transaction, Snapshot, string)
            .map(|(.., snapshot)| SetRestMore::TransactionSnapshot(snapshot)),
        (Time, Zone, zone_value)
            .map(|(.., zone)| SetRestMore::TimeZone(zone)),
        (Kw::Catalog, string)
            .right()
            .map(SetRestMore::Catalog),
        (Kw::Schema, string)
            .right()
            .map(SetRestMore::Schema),
        (Names, opt_encoding)
            .right()
            .map(SetRestMore::ClientEncoding),
        (Kw::Role, non_reserved_word_or_sconst())
            .right()    
            .map(SetRestMore::Role),
        (Xml, OptionKw, document_or_content())
            .map(|(.., option)| SetRestMore::XmlOption(option)),
        set_var_name
    )
        .parse(stream)
}

fn set_var_name(stream: &mut TokenStream) -> Result<SetRestMore> {

    let name = var_name(stream)?;

    let option =
        choice!(
            (FromKw, Current).map(|_| None),
            generic_set_tail().map(Some)
        )
        .parse(stream)?;

    let option = match option {
        None => SetRestMore::FromCurrent { name },
        Some(value) => SetRestMore::ConfigurationParameter { name, value }
    };

    Ok(option)
}

fn session_auth_user(stream: &mut TokenStream) -> Result<ValueOrDefault<Str>> {

    /*
          DEFAULT
        | NonReservedWord_or_Sconst
    */

    choice!(
        DefaultKw.map(|_| ValueOrDefault::Default),
        non_reserved_word_or_sconst().map(ValueOrDefault::Value)
    )
        .parse(stream)
}

fn zone_value(stream: &mut TokenStream) -> Result<ZoneValue> {

    /*
          DEFAULT
        | LOCAL
        | NumericOnly
        | SCONST
        | IDENT
        | INTERVAL SCONST opt_interval
        | INTERVAL '(' ICONST ')' SCONST
    */

    choice!(
        choice!(DefaultKw, Kw::Local).map(|_: Kw| Local),
        signed_number().map(Numeric),
        choice!(string, identifier)
            .map(|name: Box<str>|
                ZoneValue::String(name.into())
            ),
        (
            Kw::Interval,
            choice!(
                (string, zone_value_interval)
                    .map(|(value, range)| Interval { value, range }),
                (i32_literal_paren(), string)
                    .map(|(precision, value)|
                        Interval {
                            value,
                            range: Full { precision: Some(precision) }
                        }
                    )
            )
        )
        .right::<_, ZoneValue>()
    )
        .parse(stream)
}

fn zone_value_interval(stream: &mut TokenStream) -> Result<IntervalRange> {

    let (zone, loc) = located!(opt_interval()).parse(stream)?;

    if matches!(zone, Full { .. } | Hour | HourToMinute) {
        return Ok(zone)
    }

    let err = InvalidZoneValue.at(loc);
    Err(err.into())
}

fn opt_encoding(stream: &mut TokenStream) -> Result<ValueOrDefault<Box<str>>> {

    choice!(
        DefaultKw.map(|_| ValueOrDefault::Default),
        string.map(ValueOrDefault::Value)
    )
        .parse(stream)
        .optional()
        .map(|value| value.unwrap_or(ValueOrDefault::Default))
        .map_err(From::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        SignedNumber::IntegerConst,
        TransactionMode::ReadOnly,
        XmlNodeKind::Document,
    };
    use pg_basics::Str;
    use test_case::test_case;

    #[test_case("session characteristics as transaction read only", SetRest::SessionTransactionCharacteristics(vec![ReadOnly]))]
    #[test_case("session authorization default", SetRest::SessionAuthorization { user: ValueOrDefault::Default })]
    #[test_case("transaction snapshot 'abc'", SetRest::TransactionSnapshot("abc".into()))]
    #[test_case("transaction read only", SetRest::LocalTransactionCharacteristics(vec![ReadOnly]))]
    #[test_case("time zone default", SetRest::TimeZone(ZoneValue::Local))]
    fn test_set_rest(source: &str, expected: SetRest) {
        test_parser!(source, set_rest, expected)
    }

    #[test_case("session authorization default", SetRestMore::SessionAuthorization { user: ValueOrDefault::Default })]
    #[test_case("transaction snapshot 'abc'", SetRestMore::TransactionSnapshot("abc".into()))]
    #[test_case("time zone default", SetRestMore::TimeZone(ZoneValue::Local))]
    #[test_case("catalog 'def'", SetRestMore::Catalog("def".into()))]
    #[test_case("schema 'ghi'", SetRestMore::Schema("ghi".into()))]
    #[test_case("names default", SetRestMore::ClientEncoding(ValueOrDefault::Default))]
    #[test_case("role action", SetRestMore::Role("action".into()))]
    #[test_case("xml option document", SetRestMore::XmlOption(Document))]
    #[test_case("_var from current", SetRestMore::FromCurrent { name: vec!["_var".into()] })]
    #[test_case("_var to default", SetRestMore::ConfigurationParameter { name: vec!["_var".into()], value: ValueOrDefault::Default })]
    fn test_set_rest_more(source: &str, expected: SetRestMore) {
        test_parser!(source, set_rest_more, expected)
    }

    #[test_case("default", ValueOrDefault::Default)]
    #[test_case("numeric", ValueOrDefault::Value(Str::Static("numeric")))]
    #[test_case("'test-string'", ValueOrDefault::Value(Str::Static("test-string")))]
    fn test_session_auth_user(source: &str, expected: ValueOrDefault<Str>) {
        test_parser!(source, session_auth_user, expected)
    }

    #[test_case("default", ZoneValue::Local)]
    #[test_case("local", ZoneValue::Local)]
    #[test_case("-10", ZoneValue::Numeric(IntegerConst(-10)))]
    #[test_case("'+01:00'", ZoneValue::String("+01:00".into()))]
    #[test_case("utf8", ZoneValue::String("utf8".into()))]
    #[test_case("interval '5' hour", ZoneValue::Interval { value: "5".into(), range: Hour })]
    #[test_case("interval(3) '5'", ZoneValue::Interval { value: "5".into(), range: Full { precision: Some(3) } })]
    fn test_zone_value(source: &str, expected: ZoneValue) {
        test_parser!(source, zone_value, expected)
    }

    #[test_case("", IntervalRange::default())]
    #[test_case("hour", Hour)]
    #[test_case("hour to minute", HourToMinute)]
    fn test_zone_value_interval(source: &str, expected: IntervalRange) {
        test_parser!(source, zone_value_interval, expected)
    }

    #[test_case("default", ValueOrDefault::Default)]
    #[test_case("", ValueOrDefault::Default)]
    #[test_case("'utf8'", ValueOrDefault::Value("utf8".into()))]
    fn test_opt_encoding(source: &str, expected: ValueOrDefault<Box<str>>) {
        test_parser!(source, opt_encoding, expected)
    }
}

use crate::combinators::document_or_content;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::located;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::generic_set_tail;
use crate::combinators::i32_literal_paren;
use crate::combinators::non_reserved_word_or_sconst;
use crate::combinators::opt_interval;
use crate::combinators::signed_number;
use crate::combinators::transaction_mode_list;
use crate::combinators::var_name;
use crate::result::Optional;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::IntervalRange;
use pg_ast::IntervalRange::Full;
use pg_ast::IntervalRange::Hour;
use pg_ast::IntervalRange::HourToMinute;
use pg_ast::SetRest;
use pg_ast::SetRestMore;
use pg_ast::ValueOrDefault;
use pg_ast::ZoneValue;
use pg_ast::ZoneValue::Interval;
use pg_ast::ZoneValue::Local;
use pg_ast::ZoneValue::Numeric;
use pg_basics::Str;
use pg_elog::parser::Error::InvalidZoneValue;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Authorization;
use pg_lexer::Keyword::Characteristics;
use pg_lexer::Keyword::Current;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::Names;
use pg_lexer::Keyword::OptionKw;
use pg_lexer::Keyword::Session;
use pg_lexer::Keyword::Snapshot;
use pg_lexer::Keyword::Time;
use pg_lexer::Keyword::Transaction;
use pg_lexer::Keyword::Xml;
use pg_lexer::Keyword::Zone;
