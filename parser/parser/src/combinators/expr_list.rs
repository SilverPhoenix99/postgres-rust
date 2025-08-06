pub(super) fn expr_list(stream: &mut TokenStream) -> scan::Result<Vec<ExprNode>> {

    /*
        a_expr ( ',' a_expr )*
    */

    many!(sep = Comma, a_expr).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

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
use crate::combinators::foundation::many;
use pg_ast::ExprNode;
use pg_combinators::Combinator;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
