pub(super) fn server(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        SERVER ColId
    */

    let (_, name) = seq!(Server, col_id)
        .parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_server() {
        test_parser!(
            source = "server foo",
            parser = server,
            expected = "foo"
        )
    }
}

use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_identifier_combinators::col_id;
use pg_lexer::Keyword::Server;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
