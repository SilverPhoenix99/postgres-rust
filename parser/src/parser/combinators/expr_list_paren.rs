/// Post-condition: Vec is **Not** empty
pub(super) fn expr_list_paren() -> impl Combinator<Output = Vec<ExprNode>> {

    /*
        '(' expr_list ')'
    */

    between_paren(expr_list())
}

use crate::parser::ast_node::ExprNode;
use crate::parser::combinators::between_paren;
use crate::parser::combinators::expr_list;
use crate::parser::combinators::foundation::Combinator;
