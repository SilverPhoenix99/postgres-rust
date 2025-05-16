/// Alias: `FetchStmt`
pub(super) fn fetch_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        FETCH fetch_args
    */

    Fetch
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::RawStmt;
use postgres_parser_lexer::Keyword::Fetch;
