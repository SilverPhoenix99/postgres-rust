/// Aliases:
/// * `comment_text`
/// * `security_label`
pub(super) fn string_or_null(stream: &mut TokenStream) -> scan::Result<Option<Box<str>>> {

    or((
        string.map(Some),
        Null.map(|_| None)
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("null", None)]
    #[test_case("'abc'", Some("abc".into()))]
    fn test_string_or_null(source: &str, expected: Option<Box<str>>) {
        test_parser!(source, string_or_null, expected)
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Null;
