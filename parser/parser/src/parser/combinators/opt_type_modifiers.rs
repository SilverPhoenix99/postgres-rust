/// Post-condition: Vec **May** be empty.
pub(super) fn opt_type_modifiers() -> impl Combinator<Output = TypeModifiers> {

    /*
        ( '(' expr_list ')' )?
    */

    expr_list_paren()
        .optional()
        .map(Option::unwrap_or_default)
}

use crate::parser::combinators::expr_list_paren;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::TypeModifiers;
