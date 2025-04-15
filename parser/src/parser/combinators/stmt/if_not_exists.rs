pub(super) fn if_not_exists() -> impl Combinator<Output = bool> {

    sequence!(If, Not, Exists)
        .optional()
        .map(|opt| opt.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use crate::parser::tests::test_parser;

    #[test_case("if not exists", true)]
    #[test_case("", false)]
    #[test_case("something else", false)]
    fn test_(source: &str, expected: bool) {
        test_parser!(source, if_not_exists(), expected)
    }
}

use crate::lexer::Keyword::If;
use crate::lexer::Keyword::Exists;
use crate::lexer::Keyword::Not;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::CombinatorHelpers;
