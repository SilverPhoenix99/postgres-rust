/// Alias: `AexprConst`
pub(super) fn expr_const(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
          ICONST
        | FCONST
        | SCONST
        | BCONST
        | XCONST
        | TRUE
        | FALSE
        | NULL
        | ConstTypename SCONST (ambiguous prefix_expr)
    */

    alt!(
        const_typename.map(ExprNode::from),
        number.map(ExprNode::from),
        string.map(StringConst),
        bit_string
            .map(|(kind, value)| match kind {
                BitStringKind::Binary => BinaryStringConst(value),
                BitStringKind::Hex => HexStringConst(value),
            }),
        True.map(|_| BooleanConst(true)),
        False.map(|_| BooleanConst(false)),
        Null.map(|_| NullConst),
    ).parse(ctx)
}

/// Alias: `ConstTypename`
fn const_typename(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {
    use Keyword as K;
    use Operator as O;

    /*
          JSON SCONST
        | DOUBLE PRECISION SCONST
        | BOOLEAN SCONST
        | SMALLINT SCONST
        | ( INT | INTEGER ) SCONST
        | BIGINT SCONST
        | REAL SCONST
        | ( NUMERIC | DEC | DECIMAL ) ( type_modifiers )? SCONST
        | FLOAT ( type_modifiers )? SCONST
        | BIT ( VARYING )? ( '(' expr_list ')' )? SCONST
        | VARCHAR ( precision )? SCONST
        | ( CHAR | CHARACTER | NCHAR ) ( VARYING )? ( precision )? SCONST
        | NATIONAL ( CHAR | CHARACTER ) ( VARYING )? ( precision )? SCONST
        | TIMESTAMP ( precision )? ( with_timezone )? SCONST
        | TIME ( precision )? ( with_timezone )? SCONST
        | INTERVAL precision SCONST
        | INTERVAL SCONST ( interval )?
    */

    // Lookahead is required to disambiguate with `prefixed_expr_const`,
    // due to conflicts with the 1st keyword.

    match ctx.stream_mut().peek2()? {
        (K(Kw::Json), String(_)) => json_typecast(ctx),
        (K(Double), K(Precision)) => double_precision_typecast(ctx),
        (K(Boolean), String(_)) => bool_typecast(ctx),
        (K(Smallint), String(_)) => smallint_typecast(ctx),
        (K(Int | Integer), String(_)) => int_typecast(ctx),
        (K(Bigint), String(_)) => bigint_typecast(ctx),
        (K(Real), String(_)) => real_typecast(ctx),

        (
            K(Dec | Decimal | Kw::Numeric),
            O(OpenParenthesis) | String(_)
        ) =>
            numeric_typecast(ctx),

        (
            K(Float),
            O(OpenParenthesis) | String(_)
        ) =>
            float_typecast(ctx),

        (
            K(Kw::Bit),
            K(Varying) | O(OpenParenthesis) | String(_)
        ) =>
            bit_string_typecast(ctx),

        (
            K(Kw::Varchar),
            O(OpenParenthesis) | String(_)
        )
        | (
            K(Char | Character | Nchar),
            K(Varying) | O(OpenParenthesis) | String(_)
        )
        | (
            K(National),
            K(Char | Character)
        ) =>
            char_string_typecast(ctx),

        (
            K(Timestamp),
            O(OpenParenthesis) | K(With | Without) | String(_)
        ) =>
            timestamp_typecast(ctx),

        (
            K(Time),
            O(OpenParenthesis) | K(With | Without) | String(_)
        ) =>
            time_typecast(ctx),

        (
            K(Interval),
            O(OpenParenthesis) | String(_)
        ) =>
            interval_typecast(ctx),

        _ => no_match(ctx)
    }
}

fn json_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
        JSON SCONST
    */

    let (_, value) = seq!(skip(1), string)
        .parse(ctx)?;

    let expr = StringTypecastExpr::new(value, Json);
    Ok(expr)
}

fn double_precision_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
        DOUBLE PRECISION SCONST
    */

    let (_, value) = seq!(skip(2), string)
        .parse(ctx)?;

    let expr = StringTypecastExpr::new(value, Float8);
    Ok(expr)
}

fn bool_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
        BOOLEAN SCONST
    */

    let (_, value) = seq!(skip(1), string)
        .parse(ctx)?;

    let expr = StringTypecastExpr::new(value, Bool);
    Ok(expr)
}

fn smallint_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
        SMALLINT SCONST
    */

    let (_, value) = seq!(skip(1), string)
        .parse(ctx)?;

    let expr = StringTypecastExpr::new(value, Int2);
    Ok(expr)
}

fn int_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
        ( INT | INTEGER ) SCONST
    */

    let (_, value) = seq!(skip(1), string)
        .parse(ctx)?;

    let expr = StringTypecastExpr::new(value, Int4);
    Ok(expr)
}

fn bigint_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
        BIGINT SCONST
    */

    let (_, value) = seq!(skip(1), string)
        .parse(ctx)?;

    let expr = StringTypecastExpr::new(value, Int8);
    Ok(expr)
}

fn real_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
        REAL SCONST
    */

    let (_, value) = seq!(skip(1), string)
        .parse(ctx)?;

    let expr = StringTypecastExpr::new(value, Float4);
    Ok(expr)
}

fn numeric_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
          NUMERIC ( type_modifiers )? SCONST
        | DEC ( type_modifiers )? SCONST
        | DECIMAL ( type_modifiers )? SCONST
    */

    let (type_name, value) = seq!(numeric, string)
        .parse(ctx)
        .required()?;

    let expr = StringTypecastExpr::new(value, type_name);
    Ok(expr)
}

fn float_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
        FLOAT ( type_modifiers )? SCONST
    */

    let (type_name, value) = seq!(float, string)
        .parse(ctx)
        .required()?;

    let expr = StringTypecastExpr::new(value, type_name);
    Ok(expr)
}

fn bit_string_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
        BIT ( VARYING )? ( '(' expr_list ')' )? SCONST
    */

    let (type_name, value) = seq!(bit(None), string)
        .parse(ctx)
        .required()?;

    let expr = StringTypecastExpr::new(value, type_name);
    Ok(expr)
}

fn char_string_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
          VARCHAR ( precision )? SCONST
        | ( CHAR | CHARACTER | NCHAR ) ( VARYING )? ( precision )? SCONST
        | NATIONAL ( CHAR | CHARACTER ) ( VARYING )? ( precision )? SCONST
    */

    let (type_name, value) = seq!(character(None), string)
        .parse(ctx)
        .required()?;

    let expr = StringTypecastExpr::new(value, type_name);
    Ok(expr)
}

fn timestamp_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
        TIMESTAMP ( precision )? ( with_timezone )? SCONST
    */

    let (type_name, value) = seq!(timestamp, string)
        .parse(ctx)
        .required()?;

    let expr = StringTypecastExpr::new(value, type_name);
    Ok(expr)
}

fn time_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
        TIME ( precision )? ( with_timezone )? SCONST
    */

    let (type_name, value) = seq!(time, string)
        .parse(ctx)
        .required()?;

    let expr = StringTypecastExpr::new(value, type_name);
    Ok(expr)
}

fn interval_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    /*
          INTERVAL '(' ICONST ')' SCONST
        | INTERVAL SCONST ( interval )?
    */

    let (_, (interval, value)) = seq!(
        skip(1),
        alt!(
            seq!(
                precision
                    .map(|precision| Full { precision: Some(precision) }),
                string
            ),
            seq!(
                string,
                interval.optional()
            ).map(|(value, interval)|
                (interval.unwrap_or_default(), value)
            )
        )
    ).parse(ctx)?;

    let type_name = TypeName::Interval(interval);
    let expr = StringTypecastExpr::new(value, type_name);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::ExprNode::*;
    use pg_ast::TypeName;
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::IntervalRange::YearToMonth,
        pg_ast::TypeName::*,
        pg_basics::NumberRadix::Decimal,
    };

    #[test_case("123", IntegerConst(123))]
    #[test_case("123.45", NumericConst { radix: Decimal, value: "123.45".into() })]
    #[test_case("true", BooleanConst(true))]
    #[test_case("false", BooleanConst(false))]
    #[test_case("null", NullConst)]
    #[test_case("b'0101'", BinaryStringConst("0101".into()))]
    #[test_case("x'19af'", HexStringConst("19af".into()))]
    #[test_case("'string literal'", StringConst("string literal".into()))]
    #[test_case("double precision '1.23'", StringTypecastExpr::new("1.23", Float8).into())]
    fn test_expr_const(source: &str, expected: ExprNode) {
        test_parser!(source, expr_const, expected)
    }

    // NB: Methods using `stream.next()` cannot be tested directly with `test_parser!`.
    // NB2: A lot of cases are already tested in `simple_typename()`.
    #[test_case("json '{}'",                        Json, "{}")]
    #[test_case("double precision '1.23'",          Float8, "1.23")]
    #[test_case("boolean 'true'",                   Bool, "true")]
    #[test_case("smallint '11'",                    Int2, "11")]
    #[test_case("int '42'",                         Int4, "42")]
    #[test_case("integer '420'",                    Int4, "420")]
    #[test_case("bigint '1'",                       Int8, "1")]
    #[test_case("real '42.0'",                      Float4, "42.0")]
    #[test_case("numeric '123.45'",                 Numeric(None), "123.45")]
    #[test_case("float(25) '123.45'",               Float8, "123.45")]
    #[test_case("bit varying(6) '7'",               Varbit(Some(vec![IntegerConst(6)])), "7")]
    #[test_case("character varying 'foo'",          Varchar { max_length: None }, "foo")]
    #[test_case("timestamp with time zone 'foo'",   TimestampTz { precision: None }, "foo")]
    #[test_case("time(1) with time zone 'foo'",     TimeTz { precision: Some(1) }, "foo")]
    #[test_case("interval '1 day'",                 TypeName::Interval(Full { precision: None }), "1 day")]
    #[test_case("interval(3) '1 day'",              TypeName::Interval(Full { precision: Some(3) }), "1 day")]
    #[test_case("interval '1970-01' year to month", TypeName::Interval(YearToMonth), "1970-01")]
    fn test_const_typename(source: &str, expected_type: TypeName, value: &str) {

        let expected = StringTypecastExpr::new(value, expected_type);

        test_parser!(source, const_typename, expected)
    }
}

use crate::combinators::interval;
use crate::combinators::precision;
use crate::combinators::simple_typename::bit;
use crate::combinators::simple_typename::character;
use crate::combinators::simple_typename::float;
use crate::combinators::simple_typename::numeric;
use crate::combinators::simple_typename::time;
use crate::combinators::simple_typename::timestamp;
use crate::no_match;
use pg_ast::ExprNode;
use pg_ast::ExprNode::BinaryStringConst;
use pg_ast::ExprNode::BooleanConst;
use pg_ast::ExprNode::HexStringConst;
use pg_ast::ExprNode::NullConst;
use pg_ast::ExprNode::StringConst;
use pg_ast::IntervalRange::Full;
use pg_ast::StringTypecastExpr;
use pg_ast::TypeName;
use pg_ast::TypeName::Bool;
use pg_ast::TypeName::Float4;
use pg_ast::TypeName::Float8;
use pg_ast::TypeName::Int2;
use pg_ast::TypeName::Int4;
use pg_ast::TypeName::Int8;
use pg_ast::TypeName::Json;
use pg_combinators::alt;
use pg_combinators::bit_string;
use pg_combinators::number;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_lexer::BitStringKind;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Bigint;
use pg_lexer::Keyword::Boolean;
use pg_lexer::Keyword::Char;
use pg_lexer::Keyword::Character;
use pg_lexer::Keyword::Dec;
use pg_lexer::Keyword::Decimal;
use pg_lexer::Keyword::Double;
use pg_lexer::Keyword::False;
use pg_lexer::Keyword::Float;
use pg_lexer::Keyword::Int;
use pg_lexer::Keyword::Integer;
use pg_lexer::Keyword::Interval;
use pg_lexer::Keyword::National;
use pg_lexer::Keyword::Nchar;
use pg_lexer::Keyword::Null;
use pg_lexer::Keyword::Precision;
use pg_lexer::Keyword::Real;
use pg_lexer::Keyword::Smallint;
use pg_lexer::Keyword::Time;
use pg_lexer::Keyword::Timestamp;
use pg_lexer::Keyword::True;
use pg_lexer::Keyword::Varying;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Without;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
use pg_parser_core::stream::TokenValue::String;
use pg_parser_core::ParserContext;
use pg_parser_core::Required;
