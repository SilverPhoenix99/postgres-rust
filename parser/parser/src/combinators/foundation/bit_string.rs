
/// Aliases:
/// * `BCONST`
/// * `XCONST`
pub(in crate::combinators) fn bit_string() -> BitStringCombi {
    BitStringCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct BitStringCombi;

impl Combinator for BitStringCombi {
    type Output = (BitStringKind, Box<str>);

    /// Note that it doesn't validate the content.
    /// That needs to be done in a separate stage,
    /// when we know the actual type from the catalog.
    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output> {
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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
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

use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::BitString;
use core::mem;
use pg_lexer::BitStringKind;
