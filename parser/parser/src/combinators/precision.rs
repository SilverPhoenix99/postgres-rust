pub(super) fn precision(ctx: &mut ParserContext) -> scan::Result<i32> {
    i32_literal_paren(ctx)
}

use crate::combinators::i32_literal_paren;
use pg_combinators::ParserContext;
use pg_parser_core::scan;
