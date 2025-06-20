/// Aliases:
/// * `comment_text`
/// * `security_label`
pub(super) fn string_or_null() -> impl Combinator<Output = Option<Box<str>>> {

    parser(|stream|
        choice!(stream,
            string(stream).map(Some),
            Null.parse(stream).map(|_| None)
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("null", None)]
    #[test_case("'abc'", Some("abc".into()))]
    fn test_string_or_null(source: &str, expected: Option<Box<str>>) {
        test_parser!(source, string_or_null(), expected)
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use pg_lexer::Keyword::Null;
