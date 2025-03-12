/// Alias: `FetchStmt`
pub(super) fn move_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        MOVE fetch_args
    */

    Move
        .map(|_| todo!())
}

use crate::lexer::Keyword::Move;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
