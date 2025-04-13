pub(super) fn sequence() -> impl Combinator<Output = QualifiedName> {

    /*
        SEQUENCE any_name
    */

    Sequence
        .and_right(any_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_sequence() {
        test_parser!(
            source = "sequence foo",
            parser = sequence(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::lexer::Keyword::Sequence;
use crate::parser::ast_node::QualifiedName;
use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
