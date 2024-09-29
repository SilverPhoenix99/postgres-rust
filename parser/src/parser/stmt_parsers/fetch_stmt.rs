impl Parser<'_> {
    pub(in crate::parser) fn fetch_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Reserved(Fetch))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Reserved;
use crate::lexer::ReservedKeyword::Fetch;
use crate::parser::{AstNode, OptResult, Parser};
