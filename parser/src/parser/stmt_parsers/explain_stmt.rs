impl Parser<'_> {
    pub(in crate::parser) fn explain_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Explain))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Explain;
use crate::parser::{AstNode, OptResult, Parser};
