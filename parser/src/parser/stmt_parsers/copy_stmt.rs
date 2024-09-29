impl Parser<'_> {
    pub(in crate::parser) fn copy_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(CopyKw))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::CopyKw;
use crate::parser::{AstNode, OptResult, Parser};
