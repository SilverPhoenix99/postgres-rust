/// Alias: `DiscardStmt`
pub(super) fn discard_stmt(stream: &mut TokenStream) -> scan::Result<DiscardStmt> {

    /*
        DISCARD (ALL | PLANS | SEQUENCES | TEMP | TEMPORARY)
    */

    let (_, stmt) = seq!(=>
        Discard.parse(stream),
        choice!(stream =>
            All.parse(stream).map(|_| DiscardStmt::All),
            Plans.parse(stream).map(|_| DiscardStmt::Plans),
            Sequences.parse(stream).map(|_| DiscardStmt::Sequences),
            choice!(parsed stream => Temp, Temporary)
                .map(|_| DiscardStmt::Temporary),
        )
    )?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
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

use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::DiscardStmt;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Discard;
use pg_lexer::Keyword::Plans;
use pg_lexer::Keyword::Sequences;
use pg_lexer::Keyword::Temp;
use pg_lexer::Keyword::Temporary;
