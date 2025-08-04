/// Alias: `opt_type_modifiers`
pub(super) fn type_modifiers(stream: &mut TokenStream) -> scan::Result<TypeModifiers> {

    /*
        '(' expr_list ')'
    */

    paren!(expr_list).parse(stream)
}

use crate::combinators::expr_list;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::TypeModifiers;
