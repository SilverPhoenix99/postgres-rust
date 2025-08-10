pub fn precision(ctx: &mut ParserContext) -> scan::Result<i32> {
    i32_literal_paren(ctx)
}

use crate::i32_literal_paren;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
