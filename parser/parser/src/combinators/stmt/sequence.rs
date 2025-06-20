pub(super) fn sequence() -> impl Combinator<Output = QualifiedName> {

    /*
        SEQUENCE any_name
    */

    Sequence
        .and_right(parser(any_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_sequence() {
        test_parser!(
            source = "sequence foo",
            parser = sequence(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Sequence;
