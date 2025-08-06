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
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("$dollar$a $ string$dollar$" => Ok("a $ string".into()))]
    #[test_case("'basic string'" => Ok("basic string".into()))]
    #[test_case("'basic ''string'''\n' concatenation'" => Ok("basic 'string' concatenation".into()))]
    #[test_case(r"e'\u0061n extended string'" => Ok("an extended string".into()))]
    #[test_case("e'extended string'\n' concatenation'" => Ok("extended string concatenation".into()))]
    #[test_case(r"u&'\0061n unicode string'" => Ok("an unicode string".into()))]
    #[test_case("u&'!0061n escaped unicode string!0021' UESCAPE '!'" => Ok("an escaped unicode string!".into()))]
    #[test_case("u&'unicode string'\n' concatenation'" => Ok("unicode string concatenation".into()))]
    #[test_case("u&'*002a extended unicode *002a' UESCAPE e'*'" => Ok("* extended unicode *".into()))]
    #[test_case("u&'unicode esc!0061pe concatenation' UESCAPE ''\n''\n'!'" => Ok("unicode escape concatenation".into()))]
    fn test_string(source: &str) -> scan::Result<Box<str>> {
        test_parser!(source, string)
    }
}

use core::mem;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::stream::TokenValue;
