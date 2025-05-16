pub(super) fn table() -> impl Combinator<Output = QualifiedName> {

    /*
        TABLE any_name
    */

    Table
        .and_right(any_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_table() {
        test_parser!(
            source = "table foo",
            parser = table(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Table;
