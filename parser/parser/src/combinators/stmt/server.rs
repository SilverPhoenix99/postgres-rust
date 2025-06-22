pub(super) fn server() -> impl Combinator<Output = Str> {

    /*
        SERVER ColId
    */

    Server
        .and_right(parser(col_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_server() {
        test_parser!(
            source = "server foo",
            parser = server(),
            expected = "foo".into()
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::{parser, Combinator};
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::Str;
use pg_lexer::Keyword::Server;
