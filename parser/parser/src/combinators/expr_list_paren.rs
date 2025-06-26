pub(super) fn expr_list_paren(stream: &mut TokenStream) -> scan::Result<Vec<ExprNode>> {

    /*
        '(' expr_list ')'
    */

    between!(paren : stream => expr_list(stream))
}

use crate::combinators::expr_list;
use crate::combinators::foundation::between;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
