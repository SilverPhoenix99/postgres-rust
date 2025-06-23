pub(super) fn table(stream: &mut TokenStream) -> Result<QualifiedName> {

    /*
        TABLE any_name
    */

    seq!(stream => Table, any_name)
        .map(|(_, name)| name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_table() {
        test_parser!(
            source = "table foo",
            parser = table,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Table;
