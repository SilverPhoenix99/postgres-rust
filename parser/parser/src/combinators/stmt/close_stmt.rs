/// Alias: `ClosePortalStmt`
pub(super) fn close_stmt(stream: &mut TokenStream) -> scan::Result<OneOrAll<Str>> {

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
    ).parse(stream)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("close all", OneOrAll::All)]
    #[test_case("close abort", OneOrAll::One("abort".into()))]
    #[test_case("close ident", OneOrAll::One("ident".into()))]
    fn test_close_all(source: &str, expected: OneOrAll<Str>) {
        test_parser!(source, close_stmt, expected)
    }
}

use pg_ast::OneOrAll;
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_identifier_combinators::col_id;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Close;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
