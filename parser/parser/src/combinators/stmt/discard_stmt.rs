/// Alias: `DiscardStmt`
pub(super) fn discard_stmt() -> impl Combinator<Output = DiscardStmt> {

    /*
        DISCARD (ALL | PLANS | SEQUENCES | TEMP | TEMPORARY)
    */

    Discard
        .and_right(match_first!{
            All.map(|_| DiscardStmt::All),
            Plans.map(|_| DiscardStmt::Plans),
            Sequences.map(|_| DiscardStmt::Sequences),
            Temp.or(Temporary)
                .map(|_| DiscardStmt::Temporary),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("discard all", DiscardStmt::All)]
    #[test_case("discard plans", DiscardStmt::Plans)]
    #[test_case("discard sequences", DiscardStmt::Sequences)]
    #[test_case("discard temp", DiscardStmt::Temporary)]
    #[test_case("discard temporary", DiscardStmt::Temporary)]
    fn test_discard(source: &str, expected: DiscardStmt) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), discard_stmt().parse(&mut stream));
    }
}

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::DiscardStmt;
use postgres_parser_lexer::Keyword::All;
use postgres_parser_lexer::Keyword::Discard;
use postgres_parser_lexer::Keyword::Plans;
use postgres_parser_lexer::Keyword::Sequences;
use postgres_parser_lexer::Keyword::Temp;
use postgres_parser_lexer::Keyword::Temporary;
