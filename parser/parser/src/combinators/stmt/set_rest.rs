pub(super) fn set_rest(ctx: &mut ParserContext) -> scan::Result<SetRest> {

    /*
          SESSION CHARACTERISTICS AS TRANSACTION transaction_mode_list
        | SESSION AUTHORIZATION session_auth_user
        | TRANSACTION SNAPSHOT SCONST
        | TRANSACTION transaction_mode_list
        | set_rest_more
    */

    alt!(
        seq!(Session, set_rest_session)
            .map(|(_, stmt)| stmt),
        seq!(Transaction, set_rest_transaction)
            .map(|(_, stmt)| stmt),
        set_rest_more
            .map(SetRest::from)
    ).parse(ctx)
}

fn set_rest_session(ctx: &mut ParserContext) -> scan::Result<SetRest> {

    alt!(
        seq!(Characteristics, As, Transaction, transaction_mode_list)
            .map(|(.., modes)| SetRest::SessionTransactionCharacteristics(modes)),
        seq!(Authorization, session_auth_user)
            .map(|(_, user)| SetRest::SessionAuthorization { user })
    ).parse(ctx)
}

fn set_rest_transaction(ctx: &mut ParserContext) -> scan::Result<SetRest> {

    alt!(
        seq!(Snapshot, string)
            .map(|(_, snapshot)| SetRest::TransactionSnapshot(snapshot)),
        transaction_mode_list
            .map(SetRest::LocalTransactionCharacteristics)
    ).parse(ctx)
}

pub(super) fn set_rest_more(ctx: &mut ParserContext) -> scan::Result<SetRestMore> {

    /*
          SESSION AUTHORIZATION session_auth_user
        | TRANSACTION SNAPSHOT SCONST
        | TIME ZONE zone_value
        | CATALOG_P SCONST
        | SCHEMA SCONST
        | NAMES ( encoding )?
        | ROLE NonReservedWord_or_Sconst
        | XML_P OPTION document_or_content
        | var_name FROM CURRENT_P
        | var_name generic_set_tail
    */

    // All keywords conflict with `var_name`, so it needs to be last

    alt!(
        seq!(Session, Authorization, session_auth_user)
            .map(|(.., user)| SetRestMore::SessionAuthorization { user }),
        seq!(Transaction, Snapshot, string)
            .map(|(.., snapshot)| SetRestMore::TransactionSnapshot(snapshot)),
        seq!(Time, Zone, zone_value)
            .map(|(.., zone)| SetRestMore::TimeZone(zone)),
        seq!(Kw::Catalog, string)
            .map(|(_, catalog)| SetRestMore::Catalog(catalog)),
        seq!(Kw::Schema, string)
            .map(|(_, schema)| SetRestMore::Schema(schema)),
        seq!(Names, encoding.optional())
            .map(|(_, encoding)| SetRestMore::ClientEncoding(encoding.unwrap_or_default())),
        seq!(Kw::Role, non_reserved_word_or_sconst)
            .map(|(_, role)| SetRestMore::Role(role)),
        seq!(Xml, OptionKw, document_or_content)
            .map(|(.., option)| SetRestMore::XmlOption(option)),
        set_var_name
    ).parse(ctx)
}

fn set_var_name(ctx: &mut ParserContext) -> scan::Result<SetRestMore> {

    let name = var_name(ctx)?;

    let option = alt!(
        seq!(FromKw, Current).map(|_| None),
        generic_set_tail.map(Some)
    ).parse(ctx)?;

    let option = match option {
        None => SetRestMore::FromCurrent { name },
        Some(value) => SetRestMore::ConfigurationParameter { name, value }
    };

    Ok(option)
}

fn session_auth_user(ctx: &mut ParserContext) -> scan::Result<ValueOrDefault<Str>> {

    /*
          DEFAULT
        | NonReservedWord_or_Sconst
    */

    alt!(
        DefaultKw.map(|_| ValueOrDefault::Default),
        non_reserved_word_or_sconst.map(ValueOrDefault::Value)
    ).parse(ctx)
}

fn zone_value(ctx: &mut ParserContext) -> scan::Result<ZoneValue> {

    /*
          DEFAULT
        | LOCAL
        | NumericOnly
        | SCONST
        | IDENT
        | INTERVAL SCONST ( interval )?
        | INTERVAL '(' ICONST ')' SCONST
    */

    alt!(
        alt!(DefaultKw, Kw::Local)
            .map(|_: Kw| Local),
        signed_number.map(Numeric),
        alt!(string, identifier)
            .map(|name: Box<str>|
                ZoneValue::String(name.into())
            ),
        zone_interval
    ).parse(ctx)
}

fn zone_interval(ctx: &mut ParserContext) -> scan::Result<ZoneValue> {

    /*
        | INTERVAL SCONST ( interval )?
        | INTERVAL '(' ICONST ')' SCONST
    */

    let (_, zone) = seq!(
        Kw::Interval,
        alt!(
            seq!(string, zone_value_interval)
                .map(|(value, range)| Interval { value, range }),
            seq!(precision, string)
                .map(|(precision, value)|
                    Interval {
                        value,
                        range: Full { precision: Some(precision) }
                    }
                )
        )
    ).parse(ctx)?;

    Ok(zone)
}

fn zone_value_interval(ctx: &mut ParserContext) -> scan::Result<IntervalRange> {

    let Located(zone, loc) = located!(
        interval.optional()
            .map(Option::unwrap_or_default)
    ).parse(ctx)?;

    if matches!(zone, Full { .. } | Hour | HourToMinute) {
        return Ok(zone)
    }

    Err(InvalidZoneValue.at_location(loc).into())
}

/// Alias: `opt_encoding`
fn encoding(ctx: &mut ParserContext) -> scan::Result<ValueOrDefault<Box<str>>> {

    /*
          DEFAULT
        | SCONST
    */

    alt!(
        DefaultKw.map(|_| ValueOrDefault::Default),
        string.map(ValueOrDefault::Value)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::DefaultableValue;
    use pg_ast::SignedNumber::IntegerConst;
    use pg_ast::TransactionMode::ReadOnly;
    use pg_ast::XmlNodeKind::Document;
    use test_case::test_matrix;

    #[test_matrix("session characteristics as transaction read only" => Ok(SetRest::SessionTransactionCharacteristics(vec![ReadOnly])))]
    #[test_matrix("session authorization default" => Ok(SetRest::SessionAuthorization { user: ValueOrDefault::Default }))]
    #[test_matrix("transaction snapshot 'abc'" => Ok(SetRest::TransactionSnapshot("abc".into())))]
    #[test_matrix("transaction read only" => Ok(SetRest::LocalTransactionCharacteristics(vec![ReadOnly])))]
    #[test_matrix("time zone default" => Ok(SetRest::TimeZone(Local)))]
    fn test_set_rest(source: &str) -> scan::Result<SetRest> {
        test_parser!(source, set_rest)
    }

    #[test_matrix("session authorization default" => Ok(SetRestMore::SessionAuthorization { user: ValueOrDefault::Default }))]
    #[test_matrix("transaction snapshot 'abc'" => Ok(SetRestMore::TransactionSnapshot("abc".into())))]
    #[test_matrix("time zone default" => Ok(SetRestMore::TimeZone(Local)))]
    #[test_matrix("catalog 'def'" => Ok(SetRestMore::Catalog("def".into())))]
    #[test_matrix("schema 'ghi'" => Ok(SetRestMore::Schema("ghi".into())))]
    #[test_matrix("names default" => Ok(SetRestMore::ClientEncoding(ValueOrDefault::Default)))]
    #[test_matrix("names 'utf8'" => Ok(SetRestMore::ClientEncoding(ValueOrDefault::Value("utf8".into()))))]
    #[test_matrix("names" => Ok(SetRestMore::ClientEncoding(ValueOrDefault::Default)))]
    #[test_matrix("role action" => Ok(SetRestMore::Role("action".into())))]
    #[test_matrix("xml option document" => Ok(SetRestMore::XmlOption(Document)))]
    #[test_matrix("_var from current" => Ok(SetRestMore::FromCurrent { name: vec!["_var".into()] }))]
    #[test_matrix("_var to default" => Ok(SetRestMore::ConfigurationParameter {
        name: vec!["_var".into()],
        value: DefaultableValue::Default
    }))]
    fn test_set_rest_more(source: &str) -> scan::Result<SetRestMore> {
        test_parser!(source, set_rest_more)
    }

    #[test_matrix("default" => Ok(ValueOrDefault::Default))]
    #[test_matrix("numeric" => Ok(ValueOrDefault::Value(Str::Static("numeric"))))]
    #[test_matrix("'test-string'" => Ok(ValueOrDefault::Value(Str::Static("test-string"))))]
    fn test_session_auth_user(source: &str) -> scan::Result<ValueOrDefault<Str>> {
        test_parser!(source, session_auth_user)
    }

    #[test_matrix("default" => Ok(Local))]
    #[test_matrix("local" => Ok(Local))]
    #[test_matrix("-10" => Ok(Numeric(IntegerConst(-10))))]
    #[test_matrix("'+01:00'" => Ok(ZoneValue::String("+01:00".into())))]
    #[test_matrix("utf8" => Ok(ZoneValue::String("utf8".into())))]
    #[test_matrix("interval '5' hour" => Ok(Interval { value: "5".into(), range: Hour }))]
    #[test_matrix("interval(3) '5'" => Ok(Interval { value: "5".into(), range: Full { precision: Some(3) } }))]
    fn test_zone_value(source: &str) -> scan::Result<ZoneValue> {
        test_parser!(source, zone_value)
    }

    #[test_matrix("" => Ok(IntervalRange::default()))]
    #[test_matrix("hour" => Ok(Hour))]
    #[test_matrix("hour to minute" => Ok(HourToMinute))]
    fn test_zone_value_interval(source: &str) -> scan::Result<IntervalRange> {
        test_parser!(source, zone_value_interval)
    }

    #[test_matrix("default" => Ok(ValueOrDefault::Default))]
    #[test_matrix("'utf8'" => Ok(ValueOrDefault::Value("utf8".into())))]
    fn test_encoding(source: &str) -> scan::Result<ValueOrDefault<Box<str>>> {
        test_parser!(source, encoding)
    }
}

use crate::alt;
use crate::combinators::core::identifier;
use crate::combinators::core::string;
use crate::combinators::core::Combinator;
use crate::combinators::document_or_content;
use crate::combinators::generic_set_tail;
use crate::combinators::interval;
use crate::combinators::non_reserved_word_or_sconst;
use crate::combinators::precision;
use crate::combinators::signed_number;
use crate::combinators::stmt::transaction_mode_list;
use crate::combinators::var_name;
use crate::located;
use crate::seq;
use crate::ParserContext;
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
use pg_basics::IntoLocated;
use pg_basics::Located;
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
use pg_parser_core::scan;
