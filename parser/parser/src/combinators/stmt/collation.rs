pub(super) fn collation() -> impl Combinator<Output = QualifiedName> {

    /*
        COLLATION any_name
    */

    Collation
        .and_right(any_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_collation() {
        test_parser!(
            source = "collation foo",
            parser = collation(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_basics::QualifiedName;
use postgres_parser_lexer::Keyword::Collation;
