/// Alias: `opt_precision`
pub(super) fn precision(stream: &mut TokenStream) -> scan::Result<i32> {
    i32_literal_paren(stream)
}

use crate::combinators::i32_literal_paren;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
