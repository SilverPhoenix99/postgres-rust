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
    use crate::parser::tests::test_parser;

    #[test]
    fn test_table() {
        test_parser!(
            source = "table foo",
            parser = table(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::QualifiedName;
use postgres_parser_lexer::Keyword::Table;
