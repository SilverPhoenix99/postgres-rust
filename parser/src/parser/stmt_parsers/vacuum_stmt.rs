impl Parser<'_> {
    pub(in crate::parser) fn vacuum_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Vacuum))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Vacuum;
use crate::parser::{AstNode, OptResult, Parser};
