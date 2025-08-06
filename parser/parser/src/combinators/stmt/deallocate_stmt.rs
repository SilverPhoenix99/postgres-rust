/// Alias: `DeallocateStmt`
pub(super) fn deallocate_stmt(stream: &mut TokenStream) -> scan::Result<OneOrAll<Str>> {

    /*
        DEALLOCATE (PREPARE)? ALL
        DEALLOCATE (PREPARE)? ColId
    */

    let (.., stmt) = seq!(
        Deallocate,
        Prepare.optional(),
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

    #[test_case("deallocate all", OneOrAll::All)]
    #[test_case("deallocate prepare all", OneOrAll::All)]
    #[test_case("deallocate abort", OneOrAll::One("abort".into()))]
    #[test_case("deallocate prepare ident", OneOrAll::One("ident".into()))]
    fn test_deallocate(source: &str, expected: OneOrAll<Str>) {
        test_parser!(source, deallocate_stmt, expected)
    }
}

use crate::combinators::col_id;
use pg_ast::OneOrAll;
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Deallocate;
use pg_lexer::Keyword::Prepare;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
