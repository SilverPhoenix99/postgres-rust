pub(super) fn expr_list_paren() -> impl Combinator<Output = Vec<ExprNode>> {

    /*
        '(' expr_list ')'
    */

    between_paren(expr_list())
}

use crate::combinators::between_paren;
use crate::combinators::expr_list;
use crate::combinators::foundation::Combinator;
use pg_ast::ExprNode;
