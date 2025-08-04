pub(super) fn greatest_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        GREATEST '(' expr_list ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Kw::Greatest), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (_, args) = seq!(skip(1), paren!(expr_list))
        .parse(stream)?;

    Ok(Greatest(args))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::IntegerConst,
        scan::Error::NoMatch,
    };

    #[test_case("greatest(1, 2)" => Ok(
        Greatest(vec![
            IntegerConst(1),
            IntegerConst(2)
        ])
    ))]
    #[test_case("greatest" => matches Err(NoMatch(_)))]
    #[test_case("greatest 1" => matches Err(NoMatch(_)))]
    fn test_greatest_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, greatest_expr)
    }
}

use crate::combinators::expr_list::expr_list;
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
use pg_ast::ExprNode::Greatest;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::OpenParenthesis;
