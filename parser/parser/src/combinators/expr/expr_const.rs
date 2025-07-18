/// Alias: `AexprConst`
pub(super) fn expr_const(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
          ICONST
        | FCONST
        | SCONST
        | BCONST
        | XCONST
        | TRUE
        | FALSE
        | NULL
        | DOUBLE PRECISION SCONST (ambiguous prefix_expr)
        | ConstTypename SCONST    TODO
    */

    // Broken down into smaller combinators, due to large Rust type names.
    or((
        ambiguous_prefix_expr,

        number.map(From::from),
        string.map(StringConst),
        bit_string
            .map(|(kind, value)| match kind {
                BitStringKind::Binary => BinaryStringConst(value),
                BitStringKind::Hex => HexStringConst(value),
            }),
        True.map(|_| BooleanConst(true)),
        False.map(|_| BooleanConst(false)),
        Null.map(|_| NullConst),
    )).parse(stream)
}

fn ambiguous_prefix_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        DOUBLE PRECISION SCONST
    */

    // Lookahead is required to disambiguate with `prefixed_expr`.

    match stream.peek2()? {
        (Keyword(Double), Keyword(Precision)) => return double_precision_typecast(stream),
        (Keyword(Kw::Json), String(_)) => return json_typecast(stream),
        _ => {}
    }

    Err(NoMatch(stream.current_location()))
}

fn double_precision_typecast(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        DOUBLE PRECISION SCONST
    */

    // If it was called, then the lookahead already matched.
    stream.next(); // "double"
    stream.next(); // "precision"

    let value = string(stream).required()?;
    let expr = TypecastExpr::new(StringConst(value), Float8);
    Ok(expr.into())
}

fn json_typecast(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        JSON SCONST
    */

    // If it was called, then the lookahead already matched.
    stream.next(); // "json"

    let value = string(stream).required()?;
    let expr = TypecastExpr::new(StringConst(value), Json);
    Ok(expr.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::ExprNode::*;
    #[allow(unused_imports)]
    use pg_ast::TypeName::*;
    #[allow(unused_imports)]
    use pg_basics::NumberRadix::Decimal;
    use test_case::test_case;

    #[test_case("123", IntegerConst(123))]
    #[test_case("123.45", NumericConst { radix: Decimal, value: "123.45".into() })]
    #[test_case("true", BooleanConst(true))]
    #[test_case("false", BooleanConst(false))]
    #[test_case("null", NullConst)]
    #[test_case("b'0101'", BinaryStringConst("0101".into()))]
    #[test_case("x'19af'", HexStringConst("19af".into()))]
    #[test_case("'string literal'", StringConst("string literal".into()))]
    #[test_case("double precision '1.23'",
        TypecastExpr::new(
            StringConst("1.23".into()),
            Float8
        ).into()
    )]
    fn test_expr_const(source: &str, expected: ExprNode) {
        test_parser!(source, expr_const, expected)
    }

    // NB: Methods using `stream.next()` cannot be tested directly with `test_parser!`.
    #[test_case("double precision '1.23'",
        TypecastExpr::new(
            StringConst("1.23".into()),
            Float8
        ).into()
    )]
    #[test_case("json '{}'",
        TypecastExpr::new(
            StringConst("{}".into()),
            Json
        ).into()
    )]
    fn test_ambiguous_prefix_expr(source: &str, expected: ExprNode) {
        test_parser!(source, ambiguous_prefix_expr, expected)
    }
}

use crate::combinators::foundation::bit_string;
use crate::combinators::foundation::number;
use crate::combinators::foundation::or;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::result::Required;
use crate::scan;
use crate::scan::Error::NoMatch;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword;
use crate::stream::TokenValue::String;
use pg_ast::ExprNode;
use pg_ast::ExprNode::BinaryStringConst;
use pg_ast::ExprNode::BooleanConst;
use pg_ast::ExprNode::HexStringConst;
use pg_ast::ExprNode::NullConst;
use pg_ast::ExprNode::StringConst;
use pg_ast::TypeName::Float8;
use pg_ast::TypeName::Json;
use pg_ast::TypecastExpr;
use pg_lexer::BitStringKind;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Double;
use pg_lexer::Keyword::False;
use pg_lexer::Keyword::Null;
use pg_lexer::Keyword::Precision;
use pg_lexer::Keyword::True;
