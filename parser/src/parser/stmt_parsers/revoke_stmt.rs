impl Parser<'_> {
    pub(in crate::parser) fn revoke_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Revoke))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Revoke;
use crate::parser::{AstNode, OptResult, Parser};
