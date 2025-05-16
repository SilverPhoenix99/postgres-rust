pub(super) fn statistics() -> impl Combinator<Output = QualifiedName> {

    /*
        STATISTICS any_name
    */

    Statistics
        .and_right(any_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_statistics() {
        test_parser!(
            source = "statistics foo",
            parser = statistics(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_basics::QualifiedName;
use postgres_parser_lexer::Keyword::Statistics;
