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
    use crate::parser::tests::test_parser;

    #[test]
    fn test_collation() {
        test_parser!(
            source = "collation foo",
            parser = collation(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::lexer::Keyword::Collation;
use crate::parser::ast_node::QualifiedName;
use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
