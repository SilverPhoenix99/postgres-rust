pub(crate) fn expr_list(ctx: &mut ParserContext) -> scan::Result<Vec<ExprNode>> {

    /*
        a_expr ( ',' a_expr )*
    */

    many!(sep = Comma, a_expr).parse(ctx)
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
use pg_ast::ExprNode;
use pg_combinators::many;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
