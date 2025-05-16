/// Post-condition: Vec **May** be empty.
pub(super) fn opt_type_modifiers() -> impl Combinator<Output = TypeModifiers> {

    /*
        ( '(' expr_list ')' )?
    */

    expr_list_paren()
        .optional()
        .map(Option::unwrap_or_default)
}

use crate::combinators::expr_list_paren;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::TypeModifiers;
