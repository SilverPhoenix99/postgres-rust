impl Parser<'_> {
    pub(in crate::parser) fn call_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Call))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Call;
use crate::parser::{AstNode, OptResult, Parser};
