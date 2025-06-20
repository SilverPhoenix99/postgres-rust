pub(super) fn statistics() -> impl Combinator<Output = QualifiedName> {

    /*
        STATISTICS any_name
    */

    Statistics
        .and_right(parser(any_name))
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
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Statistics;
