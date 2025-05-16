pub(super) fn index() -> impl Combinator<Output = QualifiedName> {

    /*
        INDEX any_name
    */

    Index
        .and_right(any_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_index() {
        test_parser!(
            source = "index foo",
            parser = index(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_basics::QualifiedName;
use postgres_parser_lexer::Keyword::Index;
