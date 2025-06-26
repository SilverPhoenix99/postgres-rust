/// Aliases:
/// * `SCONST`
/// * `USCONST`
/// * `file_name`
pub(in crate::combinators) fn string(stream: &mut TokenStream<'_>) -> scan::Result<Box<str>> {
    stream.consume(|tok| {
        let TokenValue::String(value) = tok else { return None };
        Some(mem::take(value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::stream;
    use test_case::test_case;

    #[test_case("$dollar$a $ string$dollar$", "a $ string")]
    #[test_case("'basic string'", "basic string")]
    #[test_case("'basic ''string'''\n' concatenation'", "basic 'string' concatenation")]
    #[test_case(r"e'\u0061n extended string'", "an extended string")]
    #[test_case("e'extended string'\n' concatenation'", "extended string concatenation")]
    #[test_case(r"u&'\0061n unicode string'", "an unicode string")]
    #[test_case("u&'!0061n escaped unicode string!0021' UESCAPE '!'", "an escaped unicode string!")]
    #[test_case("u&'unicode string'\n' concatenation'", "unicode string concatenation")]
    #[test_case("u&'*002a extended unicode *002a' UESCAPE e'*'", "* extended unicode *")]
    #[test_case("u&'unicode esc!0061pe concatenation' UESCAPE ''\n''\n'!'", "unicode escape concatenation")]
    fn test_string(source: &str, expected: &str) {
        let mut stream = stream(source);
        let actual = string(&mut stream);
        assert_eq!(expected, actual.unwrap().as_ref())
    }
}

use crate::scan;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue;
use core::mem;
