pub(super) fn opt_type_modifiers() -> impl Combinator<Output = Option<TypeModifiers>> {

    /*
        ( '(' expr_list ')' )?
    */

    expr_list_paren()
        .optional()
}

use crate::combinators::expr_list_paren;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::TypeModifiers;
