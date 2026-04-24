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
    use test_case::test_matrix;

    #[test_matrix("deallocate all" => Ok(OneOrAll::All))]
    #[test_matrix("deallocate prepare all" => Ok(OneOrAll::All))]
    #[test_matrix("deallocate abort" => Ok(OneOrAll::One("abort".into())))]
    #[test_matrix("deallocate prepare ident" => Ok(OneOrAll::One("ident".into())))]
    fn test_deallocate(source: &str) -> scan::Result<OneOrAll<Str>> {
        test_parser!(source, deallocate_stmt)
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
