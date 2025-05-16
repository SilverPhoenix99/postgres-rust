/// Alias: `FetchStmt`
pub(super) fn move_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        MOVE fetch_args
    */

    Move
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::RawStmt;
use postgres_parser_lexer::Keyword::Move;
