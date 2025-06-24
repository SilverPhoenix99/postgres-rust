pub(super) fn database(stream: &mut TokenStream) -> Result<Str> {

    /*
        DATABASE ColId
    */

    let (_, name) = seq!(stream => Database, col_id)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_database() {
        test_parser!(
            source = "database foo",
            parser = database,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Database;
