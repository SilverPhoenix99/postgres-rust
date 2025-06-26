pub(super) fn opt_precision(stream: &mut TokenStream) -> scan::Result<Option<i32>> {
    i32_literal_paren
        .optional()
        .parse(stream)
}

use crate::combinators::foundation::Combinator;
use crate::combinators::i32_literal_paren;
use crate::scan;
use crate::stream::TokenStream;
