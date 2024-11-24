/// Alias: `FetchStmt`
pub(in crate::parser) fn move_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        MOVE fetch_args
    */

    Move
        .map(|_| todo!())
}

use crate::lexer::Keyword::Move;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
