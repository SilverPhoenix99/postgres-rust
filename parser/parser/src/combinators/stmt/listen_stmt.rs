/// Alias: `ListenStmt`
pub(super) fn listen_stmt(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        LISTEN ColId
    */

    let (_, channel) = seq!(Listen, col_id)
        .parse(ctx)?;

    Ok(channel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("listen abort", "abort".into())]
    #[test_case("listen ident", "ident".into())]
    fn test_listen_stmt(source: &str, expected: Str) {
        test_parser!(source, listen_stmt, expected)
    }
}

use crate::combinators::col_id;
use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Listen;
use pg_parser_core::scan;
