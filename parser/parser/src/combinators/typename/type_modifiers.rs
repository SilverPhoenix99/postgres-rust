/// Alias: `opt_type_modifiers`
pub(super) fn type_modifiers(ctx: &mut ParserContext) -> scan::Result<TypeModifiers> {

    /*
        '(' expr_list ')'
    */

    paren!(expr_list).parse(ctx)
}

use crate::combinators::core::Combinator;
use crate::combinators::expr_list;
use crate::paren;
use crate::ParserContext;
use pg_ast::TypeModifiers;
use pg_parser_core::scan;
