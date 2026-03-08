/// Alias: `DeallocateStmt`
pub(super) fn deallocate_stmt(ctx: &mut ParserContext) -> scan::Result<OneOrAll<Str>> {

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
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("deallocate all", OneOrAll::All)]
    #[test_case("deallocate prepare all", OneOrAll::All)]
    #[test_case("deallocate abort", OneOrAll::One("abort".into()))]
    #[test_case("deallocate prepare ident", OneOrAll::One("ident".into()))]
    fn test_deallocate(source: &str, expected: OneOrAll<Str>) {
        test_parser!(source, deallocate_stmt, expected)
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
use pg_lexer::Keyword::Deallocate;
use pg_lexer::Keyword::Prepare;
use pg_parser_core::scan;
