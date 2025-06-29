pub(super) fn expr_list_paren(stream: &mut TokenStream) -> scan::Result<Vec<ExprNode>> {

    /*
        '(' expr_list ')'
    */

    between_paren(expr_list).parse(stream)
}

use crate::combinators::expr_list;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
