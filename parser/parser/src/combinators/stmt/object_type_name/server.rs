pub(in crate::combinators::stmt) fn server(ctx: &mut ParserContext) -> scan::Result<Str> {

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
    use crate::test_parser;

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
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Server;
use pg_parser_core::scan;
