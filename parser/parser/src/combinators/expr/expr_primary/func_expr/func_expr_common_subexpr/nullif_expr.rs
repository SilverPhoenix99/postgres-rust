pub(super) fn nullif_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        NULLIF '(' a_expr ',' a_expr ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Nullif), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (_, (left, _, right)) = seq!(
        skip(1),
        paren!(seq!(a_expr, Comma, a_expr))
    ).parse(stream)?;

    let operands = Box::new((left, right));
    Ok(NullIf(operands))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::{NullConst, StringConst},
        scan::Error::NoMatch,
    };

    #[test_case("nullif(null, 'foo')" => Ok(
        NullIf(Box::new((
            NullConst,
            StringConst("foo".into())
        )))
    ))]
    #[test_case("nullif" => matches Err(NoMatch(_)))]
    #[test_case("nullif 1" => matches Err(NoMatch(_)))]
    fn test_nullif_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, nullif_expr)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::ExprNode;
use pg_ast::ExprNode::NullIf;
use pg_lexer::Keyword::Nullif;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::OpenParenthesis;
