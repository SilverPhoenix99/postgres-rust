impl Parser<'_> {
    pub(in crate::parser) fn lock_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Lock))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Lock;
use crate::parser::{AstNode, OptResult, Parser};
