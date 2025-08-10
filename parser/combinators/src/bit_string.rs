/// Aliases:
/// * `BCONST`
/// * `XCONST`
///
/// Note that it doesn't validate the content.
/// That needs to be done in a separate stage,
/// when we know the actual type from the catalog.
pub fn bit_string(ctx: &mut ParserContext) -> scan::Result<(BitStringKind, Box<str>)> {
    ctx.stream_mut().consume(|tok| {
        let BitString { kind, value } = tok else { return None };
        let value = mem::take(value);
        Some((*kind, value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("b'0110'", BitStringKind::Binary, "0110".into())]
    #[test_case("b'0110'\n'1010'\n'0101'", BitStringKind::Binary, "011010100101".into())]
    #[test_case("x'abcd'", BitStringKind::Hex, "abcd".into())]
    #[test_case("x'abcd'\n'4321'\n'f765'", BitStringKind::Hex, "abcd4321f765".into())]
    fn test_bit_string(source: &str, expected_kind: BitStringKind, expected_slice: Box<str>) {
        test_parser!(
            source,
            bit_string,
            (expected_kind, expected_slice)
        )
    }
}

use crate::ParserContext;
use core::mem;
use pg_lexer::BitStringKind;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenValue::BitString;
