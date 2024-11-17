
/// Aliases:
/// * `BCONST`
/// * `XCONST`
pub(in crate::parser) fn bit_string() -> BitStringCombi {
    BitStringCombi
}

pub(in crate::parser) struct BitStringCombi;

impl ParserFunc for BitStringCombi {
    type Output = (BitStringKind, Box<str>);
    type Error = ScanErrorKind;

    /// Note that it doesn't validate the content.
    /// That needs to be done in a separate stage,
    /// when we know the actual type from the catalog.
    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {

        let (kind, slice) = self.try_consume_bit_string(stream)?;
        let mut string = slice.to_owned();

        while let Some(suffix) = self.try_consume_string(stream).optional()? {
            string.push_str(suffix);
        }

        Ok((kind, string.into_boxed_str()))
    }
}

impl BitStringCombi {

    fn try_consume_bit_string<'src>(&self, stream: &mut TokenStream<'src>) -> ScanResult<(BitStringKind, &'src str)> {

        stream.consume_with_slice(|(tok, slice, _)| {
            let BitStringLiteral(kind) = tok else { return None };

            // strip delimiters
            let slice = &slice[2..(slice.len() - 1)];
            Some((kind, slice))
        })
    }

    fn try_consume_string<'src>(&self, stream: &mut TokenStream<'src>) -> ScanResult<&'src str> {

        stream.consume_with_slice(|(tok, slice, _)| {
            tok.string()
                .filter(|kind| kind.is_concatenable())
                .map(|kind| strip_delimiters(kind, slice))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
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

use crate::parser::result::ScanErrorKind;
use crate::{
    lexer::{
        BitStringKind,
        RawTokenKind::BitStringLiteral
    },
    parser::{
        combinators::{
            string::strip_delimiters,
            ParserFunc
        },
        result::{Optional, ScanResult},
        token_stream::{SlicedTokenConsumer, TokenStream}
    }
};
