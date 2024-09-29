impl Parser<'_> {
    pub(in crate::parser) fn move_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Move))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Move;
use crate::parser::{AstNode, OptResult, Parser};
