pub(super) fn sequence(stream: &mut TokenStream) -> Result<QualifiedName> {

    /*
        SEQUENCE any_name
    */

    seq!(stream => Sequence, any_name)
        .map(|(_, name)| name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_sequence() {
        test_parser!(
            source = "sequence foo",
            parser = sequence,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Sequence;
