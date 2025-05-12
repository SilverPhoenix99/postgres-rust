/// Aliases:
/// * `comment_text`
/// * `security_label`
pub(super) fn string_or_null() -> impl Combinator<Output = Option<Box<str>>> {

    or(
        string().map(Some),
        Null.map(|_| None)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test_case("null", None)]
    #[test_case("'abc'", Some("abc".into()))]
    fn test_string_or_null(source: &str, expected: Option<Box<str>>) {
        test_parser!(source, string_or_null(), expected)
    }
}

use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::string;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::Null;
