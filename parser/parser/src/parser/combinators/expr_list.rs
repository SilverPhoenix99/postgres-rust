/// Post-condition: Vec is **Not** empty
pub(super) fn expr_list() -> impl Combinator<Output = Vec<ExprNode>> {

    /*
        a_expr ( ',' a_expr )*
    */

    many_sep(Comma, a_expr())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

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

use crate::parser::ast_node::ExprNode;
use crate::parser::combinators::expr::a_expr;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::Combinator;
use postgres_parser_lexer::OperatorKind::Comma;
