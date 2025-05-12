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
    use crate::parser::tests::test_parser;

    #[test]
    fn test_index() {
        test_parser!(
            source = "index foo",
            parser = index(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::parser::ast_node::QualifiedName;
use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::Index;
