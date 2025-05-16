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
use pg_ast::RawStmt;
use pg_lexer::Keyword::Move;
