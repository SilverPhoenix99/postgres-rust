/// Aliases:
/// * `IDENT`
/// * `UIDENT`
pub fn identifier(stream: &mut TokenStream) -> scan::Result<Box<str>> {
    stream.consume(|tok| {
        let Identifier(ident) = tok else { return None };
        Some(mem::take(ident))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("sOmE_iDeNtIfIeR" => Ok("some_identifier".into()))]
    #[test_case(r#""quoted""# => Ok("quoted".into()))]
    #[test_case(r#"u&"d\0061ta""# => Ok("data".into()))]
    #[test_case(r#"u&"d!0061ta" UESCAPE '!'"# => Ok("data".into()))]
    fn test_identifier(source: &str) -> scan::Result<Box<str>> {
        test_parser!(source, identifier)
    }
}

use core::mem;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::stream::TokenValue::Identifier;
