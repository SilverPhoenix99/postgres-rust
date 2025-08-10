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

use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Listen;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
use pg_sink_combinators::col_id;
