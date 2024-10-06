impl Parser<'_> {
    /// Alias: `FetchStmt`
    pub(in crate::parser) fn move_stmt(&mut self) -> OptResult<AstNode> {

        /*
            FETCH fetch_args
            MOVE fetch_args
        */

        if self.buffer.consume_kw_eq(Move)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Move;
use crate::parser::AstNode;
use crate::parser::OptResult;
use crate::parser::Parser;
