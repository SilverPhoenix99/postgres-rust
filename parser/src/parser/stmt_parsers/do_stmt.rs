impl Parser<'_> {
    pub(in crate::parser) fn do_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Reserved(Do))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Reserved;
use crate::lexer::ReservedKeyword::Do;
use crate::parser::{AstNode, OptResult, Parser};
