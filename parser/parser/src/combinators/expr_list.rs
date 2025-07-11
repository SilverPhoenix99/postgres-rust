pub(super) fn expr_list(stream: &mut TokenStream) -> scan::Result<Vec<ExprNode>> {

    /*
        a_expr ( ',' a_expr )*
    */

    many_sep(Comma, a_expr).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_expr_list() {
        test_parser!(
            source = "1, 2, 3",
            parser = expr_list,
            expected = vec![
                ExprNode::IntegerConst(1),
                ExprNode::IntegerConst(2),
                ExprNode::IntegerConst(3),
            ]
        )
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_lexer::OperatorKind::Comma;
