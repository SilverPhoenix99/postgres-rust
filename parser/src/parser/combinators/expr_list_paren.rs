/// Post-condition: Vec is **Not** empty
pub(super) fn expr_list_paren() -> impl Combinator<Output = Vec<ExprNode>> {

    /*
        '(' expr_list ')'
    */

    between(
        OpenParenthesis,
        expr_list::expr_list(),
        CloseParenthesis
    )
}

use crate::lexer::OperatorKind::CloseParenthesis;
use crate::lexer::OperatorKind::OpenParenthesis;
use crate::parser::ast_node::ExprNode;
use crate::parser::combinators::expr_list;
use crate::parser::combinators::foundation::between;
use crate::parser::combinators::foundation::Combinator;
