/// Alias: `ICONST`
pub(in crate::combinators) fn integer(stream: &mut TokenStream) -> scan::Result<NonNegative> {
    stream.consume(|tok| {
        let UnsignedNumber(IntegerConst(value)) = tok else { return None };
        Some(*value)
    })
}

use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::UnsignedNumber;
use pg_ast::UnsignedNumber::IntegerConst;
use pg_basics::NonNegative;
use pg_parser_core::scan;
