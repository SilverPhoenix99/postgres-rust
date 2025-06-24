pub(super) fn opt_varying(stream: &mut TokenStream) -> Result<bool> {

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
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Varying;
