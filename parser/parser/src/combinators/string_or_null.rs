/// Aliases:
/// * `comment_text`
/// * `security_label`
///
/// The `Option` result does not come from an absence of value.
/// It returns `None` when the token is the keyword `NULL`.
pub(super) fn string_or_null(stream: &mut TokenStream) -> scan::Result<Option<Box<str>>> {

    alt!(
        string.map(Some),
        Null.map(|_| None)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("null", None)]
    #[test_case("'abc'", Some("abc".into()))]
    fn test_string_or_null(source: &str, expected: Option<Box<str>>) {
        test_parser!(source, string_or_null, expected)
    }
}

use crate::combinators::foundation::alt;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Null;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
