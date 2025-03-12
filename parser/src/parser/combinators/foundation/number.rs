/// Returns `ICONST | FCONST`.
pub(in crate::parser::combinators) fn number() -> NumberCombi {
    NumberCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser::combinators) struct NumberCombi;

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
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("1.1", UnsignedNumber::NumericConst { value: "1.1".into(), radix: crate::NumberRadix::Decimal })]
    #[test_case("11",  UnsignedNumber::IntegerConst(11.into()))]
    fn test_unsigned_number(source: &str, expected: UnsignedNumber) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = number().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::parser::ast_node::UnsignedNumber;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::TokenConsumer;
use crate::parser::token_stream::TokenStream;
use crate::parser::token_value::TokenValue;
use std::mem;
