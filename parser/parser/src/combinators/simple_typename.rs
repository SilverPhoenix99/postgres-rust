/// Alias: `SimpleTypename`
pub(super) fn simple_typename(stream: &mut TokenStream) -> scan::Result<TypeName> {

    // Broken down into smaller combinators, due to large Rust type names.
    or((
        simple_typename_1,
        simple_typename_2,
    )).parse(stream)
}

fn simple_typename_1(stream: &mut TokenStream) -> scan::Result<TypeName> {
    or((
        Kw::Json.map(|_| Json),
        Boolean.map(|_| Bool),
        Smallint.map(|_| Int2),
        Bigint.map(|_| Int8),
        Real.map(|_| Float4),
        numeric,
        int,
    )).parse(stream)
}

fn simple_typename_2(stream: &mut TokenStream) -> scan::Result<TypeName> {

    or((
        float,
        bit(Some(1)), // BitWithoutLength: `bit` defaults to `bit(1)`
        character(Some(1)), // CharacterWithoutLength: `char` defaults to `char(1)`
        timestamp,
        time,
        interval_type.map(From::from),
        generic_type
    )).parse(stream)
}

fn int(stream: &mut TokenStream) -> scan::Result<TypeName> {

    /*
        INT | INTEGER
    */

    or((Int, Integer)).parse(stream)?;
    Ok(Int4)
}

pub(super) fn numeric(stream: &mut TokenStream) -> scan::Result<TypeName> {

    /*
          NUMERIC ( '(' ICONST ')' )?
        | DEC ( '(' ICONST ')' )?
        | DECIMAL ( '(' ICONST ')' )?
    */

    let (_, typ) = (
        or((Dec, Decimal, Kw::Numeric)),
        type_modifiers
            .optional()
            .map(Numeric),
    ).parse(stream)?;

    Ok(typ)
}

/// Inlined: `opt_float`
pub(super) fn float(stream: &mut TokenStream) -> scan::Result<TypeName> {

    /*
        FLOAT ( '(' ICONST ')' )?
    */

    let (_, (precision, loc)) = (
        Float,
        located(precision.optional())
    ).parse(stream)?;

    match precision {
        None | Some(25..=53) => Ok(Float8),
        Some(1..=24) => Ok(Float4),
        Some(num @ ..=0) => {
            let err = FloatPrecisionUnderflow(num).at(loc);
            Err(err.into())
        },
        Some(num @ 54..) => {
            let err = FloatPrecisionOverflow(num).at(loc);
            Err(err.into())
        },
    }
}

/// Alias: `Bit`
///
/// Inlined:
/// * `BitWithLength`
/// * `BitWithoutLength`
pub(super) fn bit(default_type_modifiers:  Option<i32>) -> impl Combinator<Output = TypeName> {

    /*
        BIT ( VARYING )? ( '(' expr_list ')' )?
    */

    parser(move |stream| {
        let (_, varying, mut modifiers) = (
            Kw::Bit,
            Varying.optional()
                .map(|varying| varying),
            type_modifiers.optional()
        ).parse(stream)?;

        if varying.is_some() {
            return Ok(Varbit(modifiers))
        }

        modifiers = modifiers.or_else(||
            default_type_modifiers
                .map(|len| vec![IntegerConst(len)])
        );

        Ok(Bit(modifiers))
    })
}

/// Alias: `Character`
///
/// Inlined:
/// * `CharacterWithLength`
/// * `CharacterWithoutLength`
/// * `character` (lowercase rule)
pub(super) fn character(default_len: Option<i32>) -> impl Combinator<Output = TypeName> {

    /*
          VARCHAR ( precision )?
        | (CHAR | CHARACTER | NCHAR) ( VARYING )? ( precision )?
        | NATIONAL (CHAR | CHARACTER) ( VARYING )? ( precision )?
    */

    parser(move |stream| {
        let (varying, mut length) = (
            or((
                Kw::Varchar.map(|_| true),
                (
                    or((
                        Char.skip(),
                        Character.skip(),
                        Nchar.skip(),
                        (
                            National,
                            or((Char, Character))
                        )
                            .skip()
                    )),
                    Varying.optional()
                        .map(|varying| varying.is_some())
                )
                    .map(|(_, varying)| varying),
            )),
            precision.optional()
        ).parse(stream)?;

        if varying {
            return Ok(Varchar { max_length: length })
        }

        length = length.or(default_len);

        Ok(Bpchar { length })
    })
}

/// Inlined: `ConstDatetime`
pub(super) fn timestamp(stream: &mut TokenStream) -> scan::Result<TypeName> {

    /*
        TIMESTAMP ( '(' ICONST ')' )? ( with_timezone )?
    */

    let (_, precision, with_tz) = (
        Kw::Timestamp,
        precision.optional(),
        with_timezone.optional()
            .map(Option::unwrap_or_default)
    ).parse(stream)?;

    let typ = if with_tz {
        TimestampTz { precision }
    }
    else {
        Timestamp { precision }
    };

    Ok(typ)
}

/// Inlined: `ConstDatetime`
pub(super) fn time(stream: &mut TokenStream) -> scan::Result<TypeName> {

    /*
        TIMESTAMP ( '(' ICONST ')' )? ( with_timezone )?
    */

    let (_, precision, with_tz) = (
        Kw::Time,
        precision.optional(),
        with_timezone.optional()
            .map(Option::unwrap_or_default)
    )
        .parse(stream)?;

    let typ = if with_tz {
        TimeTz { precision }
    }
    else {
        Time { precision }
    };

    Ok(typ)
}

fn interval_type(stream: &mut TokenStream) -> scan::Result<IntervalRange> {

    /*
          INTERVAL '(' ICONST ')'
        | INTERVAL ( interval )?
    */

    let (_, interval) = (
        Kw::Interval,
        or((
            precision
                .map(|precision| Full { precision: Some(precision) }),
            interval.optional()
                .map(Option::unwrap_or_default)
        ))
    ).parse(stream)?;

    Ok(interval)
}

/// Alias: `GenericType`
///
/// Includes `DOUBLE PRECISION` due to conflict with `Unreserved` keywords.
fn generic_type(stream: &mut TokenStream) -> scan::Result<TypeName> {

    /*
          DOUBLE PRECISION
        | type_function_name ( attrs )? ( type_modifiers )?
    */

    // `Double` conflicts with, and has lower precedence than, any other `Keyword::Unreserved`.
    // If it's followed by `Precision`, then it's a Float8.
    // Otherwise, it's a plain `Unreserved` keyword, which can be its own User Defined Type.
    if matches!(stream.peek2(), Ok((TokenValue::Keyword(Double), TokenValue::Keyword(Precision)))) {
        stream.skip(2);
        return Ok(Float8)
    }

    let (name, type_modifiers) = (
        attrs(type_function_name),
        type_modifiers.optional()
    ).parse(stream)?;

    Ok(Generic { name, type_modifiers })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("json",                           Json)]
    #[test_case("boolean",                        Bool)]
    #[test_case("smallint",                       Int2)]
    #[test_case("int",                            Int4)]
    #[test_case("integer",                        Int4)]
    #[test_case("bigint",                         Int8)]
    #[test_case("real",                           Float4)]
    #[test_case("float",                          Float8)]
    #[test_case("float(17)",                      Float4)]
    #[test_case("float(44)",                      Float8)]
    #[test_case("double precision",               Float8)]
    #[test_case("decimal",                        Numeric(None))]
    #[test_case("decimal(10)",                    Numeric(Some(vec![IntegerConst(10)])))]
    #[test_case("dec",                            Numeric(None))]
    #[test_case("dec(20)",                        Numeric(Some(vec![IntegerConst(20)])))]
    #[test_case("numeric",                        Numeric(None))]
    #[test_case("numeric(30)",                    Numeric(Some(vec![IntegerConst(30)])))]
    #[test_case("time",                           Time { precision: None })]
    #[test_case("time(5)",                        Time { precision: Some(5) })]
    #[test_case("time without time zone",         Time { precision: None })]
    #[test_case("time(7) without time zone",      Time { precision: Some(7) })]
    #[test_case("time with time zone",            TimeTz { precision: None })]
    #[test_case("time(9) with time zone",         TimeTz { precision: Some(9) })]
    #[test_case("timestamp",                      Timestamp { precision: None })]
    #[test_case("timestamp(5)",                   Timestamp { precision: Some(5) })]
    #[test_case("timestamp without time zone",    Timestamp { precision: None })]
    #[test_case("timestamp(7) without time zone", Timestamp { precision: Some(7) })]
    #[test_case("timestamp with time zone",       TimestampTz { precision: None })]
    #[test_case("timestamp(9) with time zone",    TimestampTz { precision: Some(9) })]
    #[test_case("varchar",                        Varchar { max_length: None })]
    #[test_case("char varying",                   Varchar { max_length: None })]
    #[test_case("varchar(3)",                     Varchar { max_length: Some(3) })]
    #[test_case("char varying(5)",                Varchar { max_length: Some(5) })]
    #[test_case("character varying",              Varchar { max_length: None })]
    #[test_case("character varying(2)",           Varchar { max_length: Some(2) })]
    #[test_case("nchar varying",                  Varchar { max_length: None })]
    #[test_case("nchar varying(7)",               Varchar { max_length: Some(7) })]
    #[test_case("national char varying",          Varchar { max_length: None })]
    #[test_case("national char varying(5)",       Varchar { max_length: Some(5) })]
    #[test_case("national character varying",     Varchar { max_length: None })]
    #[test_case("national character varying(3)",  Varchar { max_length: Some(3) })]
    #[test_case("bit",                            Bit(Some(vec![IntegerConst(1)])))]
    #[test_case("bit(77)",                        Bit(Some(vec![IntegerConst(77)])))]
    #[test_case("bit varying",                    Varbit(None))]
    #[test_case("char",                           Bpchar { length: Some(1) })]
    #[test_case("char(4)",                        Bpchar { length: Some(4) })]
    #[test_case("character",                      Bpchar { length: Some(1) })]
    #[test_case("character(2)",                   Bpchar { length: Some(2) })]
    #[test_case("nchar",                          Bpchar { length: Some(1) })]
    #[test_case("nchar(9)",                       Bpchar { length: Some(9) })]
    #[test_case("national char",                  Bpchar { length: Some(1) })]
    #[test_case("national char(7)",               Bpchar { length: Some(7) })]
    #[test_case("national character",             Bpchar { length: Some(1) })]
    #[test_case("national character(8)",          Bpchar { length: Some(8) })]
    #[test_case("interval",                       IntervalRange::default().into())]
    #[test_case("interval day",                   IntervalRange::Day.into())]
    #[test_case("interval(5)",                    IntervalRange::Full { precision: Some(5) }.into())]
    #[test_case("identif.attrib",                 TypeName::Generic { name: vec!["identif".into(), "attrib".into()], type_modifiers: None })]
    #[test_case("identif(33)",                    TypeName::Generic { name: vec!["identif".into()], type_modifiers: Some(vec![IntegerConst(33)]) })]
    #[test_case("double",                         TypeName::Generic { name: vec!["double".into()], type_modifiers: None })]
    #[test_case("double.unreserved",              TypeName::Generic { name: vec!["double".into(), "unreserved".into()], type_modifiers: None })]
    #[test_case("double.unreserved(55)",          TypeName::Generic { name: vec!["double".into(), "unreserved".into()], type_modifiers: Some(vec![IntegerConst(55)]) })]
    #[test_case("full.type_func_name",            TypeName::Generic { name: vec!["full".into(), "type_func_name".into()], type_modifiers: None })]
    fn test_simple_typename(source: &str, expected: TypeName) {
        test_parser!(source, simple_typename, expected)
    }
}

use crate::combinators::attrs;
use crate::combinators::foundation::located;
use crate::combinators::foundation::or;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::interval;
use crate::combinators::precision;
use crate::combinators::type_function_name;
use crate::combinators::type_modifiers;
use crate::combinators::with_timezone;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue;
use pg_ast::ExprNode::IntegerConst;
use pg_ast::IntervalRange;
use pg_ast::IntervalRange::Full;
use pg_ast::TypeName;
use pg_ast::TypeName::Bit;
use pg_ast::TypeName::Bool;
use pg_ast::TypeName::Bpchar;
use pg_ast::TypeName::Float4;
use pg_ast::TypeName::Float8;
use pg_ast::TypeName::Generic;
use pg_ast::TypeName::Int2;
use pg_ast::TypeName::Int4;
use pg_ast::TypeName::Int8;
use pg_ast::TypeName::Json;
use pg_ast::TypeName::Numeric;
use pg_ast::TypeName::Time;
use pg_ast::TypeName::TimeTz;
use pg_ast::TypeName::Timestamp;
use pg_ast::TypeName::TimestampTz;
use pg_ast::TypeName::Varbit;
use pg_ast::TypeName::Varchar;
use pg_elog::parser::Error::FloatPrecisionOverflow;
use pg_elog::parser::Error::FloatPrecisionUnderflow;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Bigint;
use pg_lexer::Keyword::Boolean;
use pg_lexer::Keyword::Char;
use pg_lexer::Keyword::Character;
use pg_lexer::Keyword::Dec;
use pg_lexer::Keyword::Decimal;
use pg_lexer::Keyword::Double;
use pg_lexer::Keyword::Float;
use pg_lexer::Keyword::Int;
use pg_lexer::Keyword::Integer;
use pg_lexer::Keyword::National;
use pg_lexer::Keyword::Nchar;
use pg_lexer::Keyword::Precision;
use pg_lexer::Keyword::Real;
use pg_lexer::Keyword::Smallint;
use pg_lexer::Keyword::Varying;
