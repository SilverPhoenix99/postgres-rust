/// Returns `ICONST | FCONST`.
pub(in crate::combinators) fn number() -> NumberCombi {
    NumberCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct NumberCombi;

impl Combinator for NumberCombi {
    type Output = UnsignedNumber;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok| {
            let TokenValue::UnsignedNumber(value) = tok else { return None };
            Some(mem::take(value))
        })
    }
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
        let actual = number().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::ScanResult;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue;
use pg_ast::UnsignedNumber;
use std::mem;
