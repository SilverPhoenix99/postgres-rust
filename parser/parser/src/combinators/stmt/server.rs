pub(super) fn server(stream: &mut TokenStream) -> Result<Str> {

    /*
        SERVER ColId
    */

    seq!(stream => Server, col_id)
        .map(|(_, name)| name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_server() {
        test_parser!(
            source = "server foo",
            parser = server,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Server;
