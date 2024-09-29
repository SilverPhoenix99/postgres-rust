impl Parser<'_> {
    pub(in crate::parser) fn alter_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Alter))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Alter;
use crate::parser::{AstNode, OptResult, Parser};
