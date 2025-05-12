pub(super) fn opt_varying() -> impl Combinator<Output = bool> {

    /*
        ( VARYING )?
    */

    Varying
        .optional()
        .map(|varying| varying.is_some())
}

use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::Varying;
