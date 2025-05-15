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
    use crate::parser::tests::test_parser;

    #[test]
    fn test_column() {
        test_parser!(
            source = "column foo",
            parser = column(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::QualifiedName;
use postgres_parser_lexer::Keyword::Column;
