/// Returns `ICONST | FCONST`.
///
/// Alias: `I_or_F_const`
pub(in crate::combinators) fn number(stream: &mut TokenStream<'_>) -> scan::Result<UnsignedNumber> {
    stream.consume(|tok| {
        let TokenValue::UnsignedNumber(value) = tok else {
            return None
        };
        Some(mem::take(value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::DEFAULT_CONFIG;
    #[allow(unused_imports)]
    use pg_basics::NumberRadix::Decimal;
    use test_case::test_case;

    #[test_case("1.1", UnsignedNumber::NumericConst { value: "1.1".into(), radix: Decimal })]
    #[test_case("11",  UnsignedNumber::IntegerConst(11.into()))]
    fn test_unsigned_number(source: &str, expected: UnsignedNumber) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = number(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue;
use core::mem;
use pg_ast::UnsignedNumber;
use pg_parser_core::scan;
