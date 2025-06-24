pub(super) fn access_method(stream: &mut TokenStream) -> Result<Str> {

    /*
        ACCESS METHOD ColId
    */

    let (.., name) = seq!(stream => Access, Method, col_id)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

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
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Access;
use pg_lexer::Keyword::Method;
