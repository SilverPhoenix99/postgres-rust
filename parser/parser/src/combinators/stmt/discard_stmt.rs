/// Alias: `DiscardStmt`
pub(super) fn discard_stmt(ctx: &mut ParserContext) -> scan::Result<DiscardStmt> {

    /*
        DISCARD (ALL | PLANS | SEQUENCES | TEMP | TEMPORARY)
    */

    let (_, stmt) = seq!(
        Discard,
        alt!(
            All.map(|_| DiscardStmt::All),
            Plans.map(|_| DiscardStmt::Plans),
            Sequences.map(|_| DiscardStmt::Sequences),
            alt!(Temp, Temporary)
                .map(|_| DiscardStmt::Temporary),
        )
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("discard all" => Ok(DiscardStmt::All))]
    #[test_matrix("discard plans" => Ok(DiscardStmt::Plans))]
    #[test_matrix("discard sequences" => Ok(DiscardStmt::Sequences))]
    #[test_matrix("discard temp" => Ok(DiscardStmt::Temporary))]
    #[test_matrix("discard temporary" => Ok(DiscardStmt::Temporary))]
    fn test_discard(source: &str) -> scan::Result<DiscardStmt> {
        test_parser!(source, discard_stmt)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::DiscardStmt;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Discard;
use pg_lexer::Keyword::Plans;
use pg_lexer::Keyword::Sequences;
use pg_lexer::Keyword::Temp;
use pg_lexer::Keyword::Temporary;
use pg_parser_core::scan;
