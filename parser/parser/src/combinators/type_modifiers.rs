/// Alias: `opt_type_modifiers`
pub(super) fn type_modifiers(stream: &mut TokenStream) -> scan::Result<TypeModifiers> {

    /*
        '(' expr_list ')'
    */

    paren!(expr_list).parse(stream)
}

use crate::combinators::expr_list;
use pg_ast::TypeModifiers;
use pg_combinators::paren;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
