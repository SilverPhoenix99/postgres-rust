pub(super) fn index(stream: &mut TokenStream) -> Result<QualifiedName> {

    /*
        INDEX any_name
    */

    seq!(stream => Index, any_name)
        .map(|(_, name)| name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_index() {
        test_parser!(
            source = "index foo",
            parser = index,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Index;
