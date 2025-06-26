pub(super) fn opt_varying(stream: &mut TokenStream) -> scan::Result<bool> {

    /*
        ( VARYING )?
    */

    Ok(
        Varying.parse(stream)
            .optional()?
            .is_some()
    )
}

use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Varying;
