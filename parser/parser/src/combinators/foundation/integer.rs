/// Alias: `ICONST`
pub(in crate::combinators) fn integer(stream: &mut TokenStream) -> Result<NonNegative> {
    stream.consume(|tok| {
        let UnsignedNumber(IntegerConst(value)) = tok else { return None };
        Some(*value)
    })
}

use crate::scan::Result;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::UnsignedNumber;
use pg_ast::UnsignedNumber::IntegerConst;
use pg_basics::NonNegative;
