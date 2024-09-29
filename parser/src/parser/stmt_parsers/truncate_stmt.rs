impl Parser<'_> {
    pub(in crate::parser) fn truncate_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Truncate))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Truncate;
use crate::parser::{AstNode, OptResult, Parser};
