/// Alias: `opt_type_modifiers`
pub(super) fn type_modifiers(ctx: &mut ParserContext) -> scan::Result<TypeModifiers> {

    /*
        '(' expr_list ')'
    */

    paren!(expr_list).parse(ctx)
}

use crate::combinators::expr_list;
use pg_ast::TypeModifiers;
use pg_combinators::paren;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_parser_core::scan;
