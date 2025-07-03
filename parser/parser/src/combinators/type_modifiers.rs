/// Alias: `opt_type_modifiers`
pub(super) fn type_modifiers(stream: &mut TokenStream) -> scan::Result<TypeModifiers> {

    /*
        '(' expr_list ')'
    */

    expr_list_paren(stream)
}

use crate::combinators::expr_list_paren;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::TypeModifiers;
