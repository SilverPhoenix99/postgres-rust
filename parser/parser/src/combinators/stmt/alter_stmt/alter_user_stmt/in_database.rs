/// Alias: `opt_in_database`
pub(super) fn in_database(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        IN DATABASE col_id
    */

    let (.., dbname) = seq!(In, Database, col_id).parse(stream)?;
    Ok(dbname)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_in_database() {
        test_parser!(
            source = "in database db_name",
            parser = in_database,
            expected = "db_name"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Database;
use pg_lexer::Keyword::In;
