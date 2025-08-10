pub fn server(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        SERVER ColId
    */

    let (_, name) = seq!(Server, col_id)
        .parse(ctx)?;

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

use crate::col_id;
use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Server;
use pg_parser_core::scan;
