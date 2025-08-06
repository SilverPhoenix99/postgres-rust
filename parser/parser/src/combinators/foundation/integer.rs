/// Alias: `ICONST`
pub(in crate::combinators) fn integer(stream: &mut TokenStream) -> scan::Result<NonNegative> {
    stream.consume(|tok| {
        let UnsignedNumber(IntegerConst(value)) = tok else { return None };
        Some(*value)
    })
}

use pg_basics::NonNegative;
use pg_basics::UnsignedNumber::IntegerConst;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::stream::TokenValue::UnsignedNumber;
