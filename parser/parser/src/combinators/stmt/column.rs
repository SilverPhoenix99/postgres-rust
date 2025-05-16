pub(super) fn column() -> impl Combinator<Output = QualifiedName> {
    /*
        COLUMN any_name
    */

    Column
        .and_right(any_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_column() {
        test_parser!(
            source = "column foo",
            parser = column(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_basics::QualifiedName;
use postgres_parser_lexer::Keyword::Column;
