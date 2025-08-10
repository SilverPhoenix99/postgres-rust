/// Alias: `ICONST`
pub fn integer(ctx: &mut ParserContext) -> scan::Result<NonNegative> {
    ctx.stream_mut().consume(|tok| {
        let UnsignedNumber(IntegerConst(value)) = tok else { return None };
        Some(*value)
    })
}

use pg_basics::NonNegative;
use pg_basics::UnsignedNumber::IntegerConst;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenValue::UnsignedNumber;
use pg_parser_core::ParserContext;
