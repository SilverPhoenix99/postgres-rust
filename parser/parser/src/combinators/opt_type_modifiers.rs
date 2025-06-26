pub(super) fn opt_type_modifiers(stream: &mut TokenStream) -> scan::Result<Option<TypeModifiers>> {

    /*
        ( '(' expr_list ')' )?
    */

    expr_list_paren(stream)
        .optional()
        .map_err(From::from)
}

use crate::combinators::expr_list_paren;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::TypeModifiers;
