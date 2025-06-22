pub(super) fn expr_list() -> impl Combinator<Output = Vec<ExprNode>> {

    /*
        a_expr ( ',' a_expr )*
    */

    many!(sep = Comma, a_expr())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_expr_list() {
        let mut stream = TokenStream::new("1, 2, 3", DEFAULT_CONFIG);

        let expected = vec![
            ExprNode::IntegerConst(1),
            ExprNode::IntegerConst(2),
            ExprNode::IntegerConst(3),
        ];

        assert_eq!(Ok(expected), expr_list().parse(&mut stream));
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::many;
use crate::combinators::foundation::Combinator;
use pg_ast::ExprNode;
use pg_lexer::OperatorKind::Comma;
