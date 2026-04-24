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
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("close all" => Ok(OneOrAll::All))]
    #[test_matrix("close abort" => Ok(OneOrAll::One("abort".into())))]
    #[test_matrix("close ident" => Ok(OneOrAll::One("ident".into())))]
    fn test_close_all(source: &str) -> scan::Result<OneOrAll<Str>> {
        test_parser!(source, close_stmt)
    }
}

use crate::alt;
use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::OneOrAll;
use pg_basics::Str;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Close;
use pg_parser_core::scan;
