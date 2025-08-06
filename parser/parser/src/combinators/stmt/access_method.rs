pub(super) fn access_method(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        ACCESS METHOD ColId
    */

    let (.., name) = seq!(Access, Method, col_id).parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_access_method() {
        test_parser!(
            source = "access method foo",
            parser = access_method,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Access;
use pg_lexer::Keyword::Method;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
