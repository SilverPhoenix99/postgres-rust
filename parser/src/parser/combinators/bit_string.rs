
/// Aliases:
/// * `BCONST`
/// * `XCONST`
pub(in crate::parser) fn bit_string() -> BitStringCombi {
    BitStringCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct BitStringCombi;

impl Combinator for BitStringCombi {
    type Output = (BitStringKind, Box<str>);

    /// Note that it doesn't validate the content.
    /// That needs to be done in a separate stage,
    /// when we know the actual type from the catalog.
    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok| {
            let BitString { kind, value } = tok else { return None };
            let value = mem::take(value);
            Some((*kind, value))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("b'0110'", BitStringKind::Binary, "0110")]
    #[test_case("b'0110'\n'1010'\n'0101'", BitStringKind::Binary, "011010100101")]
    #[test_case("x'abcd'", BitStringKind::Hex, "abcd")]
    #[test_case("x'abcd'\n'4321'\n'f765'", BitStringKind::Hex, "abcd4321f765")]
    fn test_bit_string(source: &str, expected_kind: BitStringKind, expected_slice: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let (actual_kind, actual_slice) = bit_string().parse(&mut stream).unwrap();
        assert_eq!(expected_kind, actual_kind);
        assert_eq!(expected_slice, actual_slice.as_ref());
    }
}

use crate::lexer::BitStringKind;
use crate::parser::combinators::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::TokenConsumer;
use crate::parser::token_stream::TokenStream;
use crate::parser::token_value::TokenValue::BitString;
use std::mem;
