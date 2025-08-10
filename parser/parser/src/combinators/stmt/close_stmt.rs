/// Alias: `ClosePortalStmt`
pub(super) fn close_stmt(ctx: &mut ParserContext) -> scan::Result<OneOrAll<Str>> {

    /*
          CLOSE ALL
        | CLOSE ColId
    */

    let (_, stmt) = seq!(
        Close,
        alt!(
            All.map(|_| OneOrAll::All),
            col_id.map(OneOrAll::One)
        )
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use pg_sink_ast::OneOrAll;
    use test_case::test_case;

    #[test_case("close all", OneOrAll::All)]
    #[test_case("close abort", OneOrAll::One("abort".into()))]
    #[test_case("close ident", OneOrAll::One("ident".into()))]
    fn test_close_all(source: &str, expected: OneOrAll<Str>) {
        test_parser!(source, close_stmt, expected)
    }
}

use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Close;
use pg_parser_core::scan;
use pg_sink_ast::OneOrAll;
use pg_sink_combinators::col_id;
