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
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("listen abort", "abort".into())]
    #[test_case("listen ident", "ident".into())]
    fn test_listen_stmt(source: &str, expected: Str) {
        test_parser!(source, listen_stmt, expected)
    }
}

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Listen;
use pg_parser_core::scan;
