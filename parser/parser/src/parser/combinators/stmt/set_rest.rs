pub(super) fn set_rest() -> impl Combinator<Output = SetRest> {

    /*
          SESSION CHARACTERISTICS AS TRANSACTION transaction_mode_list
        | SESSION AUTHORIZATION session_auth_user
        | TRANSACTION SNAPSHOT SCONST
        | TRANSACTION transaction_mode_list
        | set_rest_more
    */

    match_first! {
        Session.and_right(match_first! {
            sequence!(Characteristics, As, Transaction).and_right(transaction_mode_list())
                .map(SetRest::SessionTransactionCharacteristics),
            Authorization.and_right(session_auth_user())
                .map(|user| SetRest::SessionAuthorization { user })
        }),
        Transaction.and_right(match_first! {
            Snapshot.and_right(string())
                .map(SetRest::TransactionSnapshot),
            transaction_mode_list()
                .map(SetRest::LocalTransactionCharacteristics)
        }),
        set_rest_more().map(From::from)
    }
}

pub(super) fn set_rest_more() -> impl Combinator<Output = SetRestMore> {

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

    match_first! {
        sequence!(Session, Authorization)
            .and_right(session_auth_user())
            .map(|user| SetRestMore::SessionAuthorization { user }),
        sequence!(Transaction, Snapshot)
            .and_right(string())
            .map(SetRestMore::TransactionSnapshot),
        sequence!(Time, Zone)
            .and_right(zone_value())
            .map(SetRestMore::TimeZone),
        Kw::Catalog.and_right(string())
            .map(SetRestMore::Catalog),
        Kw::Schema.and_right(string())
            .map(SetRestMore::Schema),
        Names.and_right(opt_encoding())
            .map(SetRestMore::ClientEncoding),
        Kw::Role.and_right(non_reserved_word_or_sconst())
            .map(SetRestMore::Role),
        sequence!(Xml, OptionKw)
            .and_right(document_or_content())
            .map(SetRestMore::XmlOption),
        var_name().chain(match_first_with_state!(|name, stream| {
            sequence!(FromKw, Current) => (_) SetRestMore::FromCurrent { name },
            generic_set_tail() => (value) SetRestMore::ConfigurationParameter { name, value }
        }))
    }
}

fn session_auth_user() -> impl Combinator<Output = ValueOrDefault<Str>> {

    /*
          DEFAULT
        | NonReservedWord_or_Sconst
    */

    DefaultKw.map(|_| ValueOrDefault::Default)
        .or(non_reserved_word_or_sconst().map(ValueOrDefault::Value))
}

fn zone_value() -> impl Combinator<Output = ZoneValue> {

    /*
          DEFAULT
        | LOCAL
        | NumericOnly
        | SCONST
        | IDENT
        | INTERVAL SCONST opt_interval
        | INTERVAL '(' ICONST ')' SCONST
    */

    match_first! {
        DefaultKw.or(Kw::Local).map(|_| Local),
        signed_number().map(Numeric),
        or(string(), identifier()).map(|name| ZoneValue::String(name.into())),
        Kw::Interval.and_right(match_first! {
            string().and_then(zone_value_interval(), |value, range|
                Interval { value, range }
            ),
            i32_literal_paren().and_then(string(), |precision, value|
                Interval {
                    value,
                    range: Full { precision: Some(precision) }
                }
            )
        })
    }
}

fn zone_value_interval() -> impl Combinator<Output = IntervalRange> {

    located(opt_interval())
        .map_result(|res| {
            match res {
                Ok((ok @ (Full { .. } | Hour | HourToMinute), _)) => Ok(ok),
                Ok((_, loc)) => {
                    let err = ParserError::new(InvalidZoneValue, loc);
                    Err(err.into())
                },
                Err(err) => Err(err),
            }
        })
}

fn opt_encoding() -> impl Combinator<Output = ValueOrDefault<Box<str>>> {

    DefaultKw.map(|_| ValueOrDefault::Default)
        .or(string().map(ValueOrDefault::Value))
        .optional()
        .map(|value| value.unwrap_or(ValueOrDefault::Default))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use postgres_basics::Str;
    #[allow(unused_imports)]
    use postgres_parser_ast::{
        SignedNumber::IntegerConst,
        TransactionMode::ReadOnly,
        XmlNodeKind::Document,
    };
    use test_case::test_case;

    #[test_case("session characteristics as transaction read only", SetRest::SessionTransactionCharacteristics(vec![ReadOnly]))]
    #[test_case("session authorization default", SetRest::SessionAuthorization { user: ValueOrDefault::Default })]
    #[test_case("transaction snapshot 'abc'", SetRest::TransactionSnapshot("abc".into()))]
    #[test_case("transaction read only", SetRest::LocalTransactionCharacteristics(vec![ReadOnly]))]
    #[test_case("time zone default", SetRest::TimeZone(ZoneValue::Local))]
    fn test_set_rest(source: &str, expected: SetRest) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = set_rest().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
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
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = set_rest_more().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }

    #[test_case("default", ValueOrDefault::Default)]
    #[test_case("numeric", ValueOrDefault::Value(Str::Static("numeric")))]
    #[test_case("'test-string'", ValueOrDefault::Value(Str::Static("test-string")))]
    fn test_session_auth_user(source: &str, expected: ValueOrDefault<Str>) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = session_auth_user().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }

    #[test_case("default", ZoneValue::Local)]
    #[test_case("local", ZoneValue::Local)]
    #[test_case("-10", ZoneValue::Numeric(IntegerConst(-10)))]
    #[test_case("'+01:00'", ZoneValue::String("+01:00".into()))]
    #[test_case("utf8", ZoneValue::String("utf8".into()))]
    #[test_case("interval '5' hour", ZoneValue::Interval { value: "5".into(), range: Hour })]
    #[test_case("interval(3) '5'", ZoneValue::Interval { value: "5".into(), range: Full { precision: Some(3) } })]
    fn test_zone_value(source: &str, expected: ZoneValue) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = zone_value().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }

    #[test_case("", IntervalRange::default())]
    #[test_case("hour", Hour)]
    #[test_case("hour to minute", HourToMinute)]
    fn test_zone_value_interval(source: &str, expected: IntervalRange) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = zone_value_interval().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }

    #[test_case("default", ValueOrDefault::Default)]
    #[test_case("", ValueOrDefault::Default)]
    #[test_case("'utf8'", ValueOrDefault::Value("utf8".into()))]
    fn test_opt_encoding(source: &str, expected: ValueOrDefault<Box<str>>) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = opt_encoding().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::parser::combinators::document_or_content;
use crate::parser::combinators::foundation::identifier;
use crate::parser::combinators::foundation::located;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::string;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::generic_set_tail;
use crate::parser::combinators::i32_literal_paren;
use crate::parser::combinators::non_reserved_word_or_sconst;
use crate::parser::combinators::opt_interval;
use crate::parser::combinators::signed_number;
use crate::parser::combinators::transaction_mode_list;
use crate::parser::combinators::var_name;
use elog::parser::ParserError;
use elog::parser::ParserErrorKind::InvalidZoneValue;
use postgres_basics::Str;
use postgres_parser_ast::IntervalRange;
use postgres_parser_ast::IntervalRange::Full;
use postgres_parser_ast::IntervalRange::Hour;
use postgres_parser_ast::IntervalRange::HourToMinute;
use postgres_parser_ast::SetRest;
use postgres_parser_ast::SetRestMore;
use postgres_parser_ast::ValueOrDefault;
use postgres_parser_ast::ZoneValue;
use postgres_parser_ast::ZoneValue::Interval;
use postgres_parser_ast::ZoneValue::Local;
use postgres_parser_ast::ZoneValue::Numeric;
use postgres_parser_lexer::Keyword as Kw;
use postgres_parser_lexer::Keyword::As;
use postgres_parser_lexer::Keyword::Authorization;
use postgres_parser_lexer::Keyword::Characteristics;
use postgres_parser_lexer::Keyword::Current;
use postgres_parser_lexer::Keyword::DefaultKw;
use postgres_parser_lexer::Keyword::FromKw;
use postgres_parser_lexer::Keyword::Names;
use postgres_parser_lexer::Keyword::OptionKw;
use postgres_parser_lexer::Keyword::Session;
use postgres_parser_lexer::Keyword::Snapshot;
use postgres_parser_lexer::Keyword::Time;
use postgres_parser_lexer::Keyword::Transaction;
use postgres_parser_lexer::Keyword::Xml;
use postgres_parser_lexer::Keyword::Zone;
