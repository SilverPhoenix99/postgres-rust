pub(super) fn server() -> impl Combinator<Output = Str> {

    /*
        SERVER ColId
    */

    Server
        .and_right(col_id())
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
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
use postgres_parser_lexer::Keyword::Server;
