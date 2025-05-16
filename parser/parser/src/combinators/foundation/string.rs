/// Aliases:
/// * `SCONST`
/// * `USCONST`
/// * `file_name`
pub(in crate::combinators) fn string() -> StringCombi {
    StringCombi
}

// (SCONST)* as long as they're concatenable.
// Internally used on productions that don't use UESCAPE.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct StringCombi;

impl Combinator for StringCombi {
    type Output = Box<str>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok| {
            let TokenValue::String(value) = tok else { return None };
            Some(mem::take(value))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::DEFAULT_CONFIG;
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
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let parser = string();
        let actual = parser.parse(&mut stream);
        assert_eq!(expected, actual.unwrap().as_ref())
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::ScanResult;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue;
use std::mem;
