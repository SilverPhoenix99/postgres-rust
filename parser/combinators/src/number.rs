/// Returns `ICONST | FCONST`.
///
/// Alias: `I_or_F_const`
pub fn number(ctx: &mut ParserContext) -> scan::Result<UnsignedNumber> {
    ctx.stream_mut().consume(|tok| {
        let TokenValue::UnsignedNumber(value) = tok else {
            return None
        };
        Some(mem::take(value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_basics::NumberRadix::Decimal;
    use test_case::test_case;

    #[test_case("1.1" => Ok(UnsignedNumber::NumericConst { value: "1.1".into(), radix: Decimal }))]
    #[test_case("11" => Ok(UnsignedNumber::IntegerConst(11.into())))]
    fn test_unsigned_number(source: &str) -> scan::Result<UnsignedNumber> {
        test_parser!(source, number)
    }
}

use core::mem;
use pg_basics::UnsignedNumber;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenValue;
use pg_parser_core::ParserContext;
