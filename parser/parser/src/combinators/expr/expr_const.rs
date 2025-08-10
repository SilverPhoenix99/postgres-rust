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
        | FLOAT ( precision )? SCONST
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

        (
            K(Interval),
            O(OpenParenthesis) | String(_)
        ) =>
            interval_typecast(ctx),

        (
            K(Kw::Json | Boolean | Smallint | Int | Integer | Bigint | Real),
            String(_)
        )
        | (K(Double), K(Precision))
        | (
            K(Float | Dec | Decimal | Kw::Numeric | Kw::Varchar),
            O(OpenParenthesis) | String(_)
        )
        |(
            K(Kw::Bit | Char | Character | Nchar),
            K(Varying) | O(OpenParenthesis) | String(_)
        )
        | (
            K(National),
            K(Char | Character)
        )
        | (
            K(Timestamp | Time),
            K(With | Without) | O(OpenParenthesis) | String(_)
        ) =>
            simple_typecast(ctx),

        _ => no_match(ctx)
    }
}

fn simple_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    // Just offload parsing to the `simple_typename` combinator.

    let (type_name, value) = seq!(simple_typename, string)
        .parse(ctx)?;

    let expr = StringTypecastExpr::new(value, type_name);
    Ok(expr)
}

fn interval_typecast(ctx: &mut ParserContext) -> scan::Result<StringTypecastExpr> {

    // Format is not compatible with `simple_typename` combinator.

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
    use crate::combinators::expr_list;
    use pg_ast::ExprNode::*;
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::TypeName::*,
        pg_basics::NumberRadix::Decimal,
        pg_interval_ast::IntervalRange::YearToMonth,
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
    #[test_case("json '{}'"                        => Ok(StringTypecastExpr::new("{}", Json)))]
    #[test_case("double precision '1.23'"          => Ok(StringTypecastExpr::new("1.23", Float8)))]
    #[test_case("boolean 'true'"                   => Ok(StringTypecastExpr::new("true", Bool)))]
    #[test_case("smallint '11'"                    => Ok(StringTypecastExpr::new("11", Int2)))]
    #[test_case("int '42'"                         => Ok(StringTypecastExpr::new("42", Int4)))]
    #[test_case("integer '420'"                    => Ok(StringTypecastExpr::new("420", Int4)))]
    #[test_case("bigint '1'"                       => Ok(StringTypecastExpr::new("1", Int8)))]
    #[test_case("real '42.0'"                      => Ok(StringTypecastExpr::new("42.0", Float4)))]
    #[test_case("numeric '123.45'"                 => Ok(StringTypecastExpr::new("123.45", Numeric(None))))]
    #[test_case("float(25) '123.45'"               => Ok(StringTypecastExpr::new("123.45", Float8)))]
    #[test_case("bit varying(6) '7'"               => Ok(StringTypecastExpr::new("7", Varbit(Some(vec![IntegerConst(6)])))))]
    #[test_case("character varying 'foo'"          => Ok(StringTypecastExpr::new("foo", Varchar { max_length: None })))]
    #[test_case("timestamp with time zone 'foo'"   => Ok(StringTypecastExpr::new("foo", TimestampTz { precision: None })))]
    #[test_case("time(1) with time zone 'foo'"     => Ok(StringTypecastExpr::new("foo", TimeTz { precision: Some(1) })))]
    #[test_case("interval '1 day'"                 => Ok(StringTypecastExpr::new("1 day", TypeName::Interval(Full { precision: None }))))]
    #[test_case("interval(3) '1 day'"              => Ok(StringTypecastExpr::new("1 day", TypeName::Interval(Full { precision: Some(3) }))))]
    #[test_case("interval '1970-01' year to month" => Ok(StringTypecastExpr::new("1970-01", TypeName::Interval(YearToMonth))))]
    fn test_const_typename(source: &str) -> scan::Result<StringTypecastExpr> {
        let mut ctx = ParserContext::new(source, expr_list);
        const_typename(&mut ctx)
    }
}

use crate::no_match;
use pg_ast::ExprNode;
use pg_ast::ExprNode::BinaryStringConst;
use pg_ast::ExprNode::BooleanConst;
use pg_ast::ExprNode::HexStringConst;
use pg_ast::ExprNode::NullConst;
use pg_ast::ExprNode::StringConst;
use pg_ast::StringTypecastExpr;
use pg_ast::TypeName;
use pg_combinators::alt;
use pg_combinators::bit_string;
use pg_combinators::number;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_interval_ast::IntervalRange::Full;
use pg_interval_combinators::interval;
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
use pg_sink_combinators::precision;
use pg_type_combinators::simple_typename;
