impl Parser<'_> {
    pub(in crate::parser) fn set_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Set))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Set;
use crate::parser::{AstNode, OptResult, Parser};
