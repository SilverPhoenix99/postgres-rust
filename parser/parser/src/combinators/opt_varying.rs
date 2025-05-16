pub(super) fn opt_varying() -> impl Combinator<Output = bool> {

    /*
        ( VARYING )?
    */

    Varying
        .optional()
        .map(|varying| varying.is_some())
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::Varying;
