/// Alias: `DiscardStmt`
pub(super) fn discard_stmt(stream: &mut TokenStream) -> scan::Result<DiscardStmt> {

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
    ).parse(stream)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("discard all", DiscardStmt::All)]
    #[test_case("discard plans", DiscardStmt::Plans)]
    #[test_case("discard sequences", DiscardStmt::Sequences)]
    #[test_case("discard temp", DiscardStmt::Temporary)]
    #[test_case("discard temporary", DiscardStmt::Temporary)]
    fn test_discard(source: &str, expected: DiscardStmt) {
        test_parser!(source, discard_stmt, expected)
    }
}

use crate::combinators::foundation::alt;
use pg_ast::DiscardStmt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Discard;
use pg_lexer::Keyword::Plans;
use pg_lexer::Keyword::Sequences;
use pg_lexer::Keyword::Temp;
use pg_lexer::Keyword::Temporary;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
