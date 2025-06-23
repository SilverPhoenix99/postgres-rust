pub(super) fn tablespace(stream: &mut TokenStream) -> Result<Str> {

    /*
        TABLESPACE ColId
    */

    seq!(stream => Tablespace, col_id)
        .map(|(_, name)| name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_tablespace() {
        test_parser!(
            source = "tablespace foo",
            parser = tablespace,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Tablespace;
