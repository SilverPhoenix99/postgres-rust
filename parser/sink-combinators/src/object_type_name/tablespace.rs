pub fn tablespace(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        TABLESPACE ColId
    */

    let (_, name) = seq!(Tablespace, col_id)
        .parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_tablespace() {
        test_parser!(
            source = "tablespace foo",
            parser = tablespace,
            expected = "foo"
        )
    }
}

use crate::col_id;
use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Tablespace;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
