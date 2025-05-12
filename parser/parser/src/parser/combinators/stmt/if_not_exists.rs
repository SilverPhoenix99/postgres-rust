pub(super) fn if_not_exists() -> impl Combinator<Output = bool> {

    sequence!(If, Not, Exists)
        .optional()
        .map(|opt| opt.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test_case("if not exists", true)]
    #[test_case("", false)]
    #[test_case("something else", false)]
    fn test_(source: &str, expected: bool) {
        test_parser!(source, if_not_exists(), expected)
    }
}

use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::Exists;
use postgres_parser_lexer::Keyword::If;
use postgres_parser_lexer::Keyword::Not;
