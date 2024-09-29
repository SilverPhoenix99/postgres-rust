impl Parser<'_> {
    pub(in crate::parser) fn drop_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(DropKw))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::DropKw;
use crate::parser::{AstNode, OptResult, Parser};
