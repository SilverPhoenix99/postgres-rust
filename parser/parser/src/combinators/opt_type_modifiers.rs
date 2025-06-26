pub(super) fn opt_type_modifiers(stream: &mut TokenStream) -> scan::Result<Option<TypeModifiers>> {

    /*
        ( '(' expr_list ')' )?
    */

    expr_list_paren()
        .optional()
        .parse(stream)
}

use crate::combinators::expr_list_paren;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::TypeModifiers;
