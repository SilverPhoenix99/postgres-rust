/// Aliases:
/// * `IDENT`
/// * `UIDENT`
pub(in crate::combinators) fn identifier(stream: &mut TokenStream) -> Result<Box<str>> {
    stream.consume(|tok| {
        let Identifier(ident) = tok else { return None };
        Some(mem::take(ident))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("sOmE_iDeNtIfIeR", "some_identifier")]
    #[test_case(r#""quoted""#, "quoted")]
    #[test_case(r#"u&"d\0061ta""#, "data")]
    #[test_case(r#"u&"d!0061ta" UESCAPE '!'"#, "data")]
    fn test_identifier(source: &str, expected: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = identifier(&mut stream);
        assert_eq!(expected, actual.unwrap().as_ref())
    }
}

use crate::scan::Result;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Identifier;
use core::mem;
