pub(super) fn treat_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    use crate::stream::TokenValue::Keyword as K;
    use crate::stream::TokenValue::Operator as Op;

    /*
        TREAT '(' a_expr AS Typename ')'

        Converts the expression of a particular type to a target type,
        which is defined to be a subtype of the original expression.
        In SQL99, this is intended for use with structured UDTs,
        but let's make this a generally useful form allowing stronger
        coercions than are handled by implicit casting.
    */

    if ! matches!(stream.peek2(), Ok((K(Kw::Treat), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (expr, _, typename) = skip_prefix(1,
        paren((a_expr, As, typename))
    ).parse(stream)?;

    let cast = TypecastExpr::new(expr, typename);
    let expr = Treat(Box::new(cast));
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::IntegerConst,
        pg_ast::TypeName::Int4,
        scan::Error::NoMatch,
    };

    #[test_case("treat(123 as int)" => Ok(
        Treat(Box::new(
            TypecastExpr::new(
                IntegerConst(123),
                Int4
            )
        ))
    ))]
    #[test_case("treat" => matches Err(NoMatch(_)))]
    #[test_case("treat 1" => matches Err(NoMatch(_)))]
    fn test_treat_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, treat_expr)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::combinators::typename;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_ast::ExprNode::Treat;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::As;
use pg_lexer::OperatorKind::OpenParenthesis;
