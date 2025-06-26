pub(super) fn opt_precision(stream: &mut TokenStream) -> scan::Result<Option<i32>> {
    i32_literal_paren(stream)
        .optional()
        .map_err(From::from)
}

use crate::combinators::i32_literal_paren;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
