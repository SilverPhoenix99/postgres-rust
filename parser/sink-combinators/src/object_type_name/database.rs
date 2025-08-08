pub fn database(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        DATABASE ColId
    */

    let (_, name) = seq!(Database, col_id).parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_database() {
        test_parser!(
            source = "database foo",
            parser = database,
            expected = "foo"
        )
    }
}

use crate::col_id;
use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Database;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
