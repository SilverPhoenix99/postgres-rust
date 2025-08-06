/// Aliases:
/// * `IDENT`
/// * `UIDENT`
pub(in crate::combinators) fn identifier(stream: &mut TokenStream) -> scan::Result<Box<str>> {
    stream.consume(|tok| {
        let Identifier(ident) = tok else { return None };
        Some(mem::take(ident))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use crate::tests::test_parser;

    #[test_case("sOmE_iDeNtIfIeR" => Ok("some_identifier".into()))]
    #[test_case(r#""quoted""# => Ok("quoted".into()))]
    #[test_case(r#"u&"d\0061ta""# => Ok("data".into()))]
    #[test_case(r#"u&"d!0061ta" UESCAPE '!'"# => Ok("data".into()))]
    fn test_identifier(source: &str) -> scan::Result<Box<str>> {
        test_parser!(source, identifier)
    }
}

use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Identifier;
use core::mem;
use pg_parser_core::scan;
