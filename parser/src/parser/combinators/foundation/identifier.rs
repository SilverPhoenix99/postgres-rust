/// Aliases:
/// * `IDENT`
/// * `UIDENT`
pub(in crate::parser::combinators) fn identifier() -> IdentifierCombi {
    IdentifierCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser::combinators) struct IdentifierCombi;

impl Combinator for IdentifierCombi {
    type Output = Box<str>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok| {
            let Identifier(ident) = tok else { return None };
            Some(mem::take(ident))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("sOmE_iDeNtIfIeR", "some_identifier")]
    #[test_case(r#""quoted""#, "quoted")]
    #[test_case(r#"u&"d\0061ta""#, "data")]
    #[test_case(r#"u&"d!0061ta" UESCAPE '!'"#, "data")]
    fn test_identifier(source: &str, expected: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let parser = identifier();
        let actual = parser.parse(&mut stream);
        assert_eq!(expected, actual.unwrap().as_ref())
    }
}

use crate::parser::combinators::foundation::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::TokenConsumer;
use crate::parser::token_stream::TokenStream;
use crate::parser::token_value::TokenValue::Identifier;
use std::mem;
