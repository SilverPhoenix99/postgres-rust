impl Parser<'_> {
    pub(in crate::parser) fn cluster_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Cluster))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Cluster;
use crate::parser::{AstNode, OptResult, Parser};
