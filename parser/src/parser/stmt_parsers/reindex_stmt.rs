impl Parser<'_> {
    pub(in crate::parser) fn reindex_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Reindex))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Reindex;
use crate::parser::{AstNode, OptResult, Parser};
