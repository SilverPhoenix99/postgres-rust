impl Parser<'_> {
    pub(in crate::parser) fn prepare_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Prepare))?.is_none() {
            return Ok(None)
        }

        let name = self.col_id().required()?;

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Prepare;
use crate::parser::result::OptionalResult;
use crate::parser::{AstNode, OptResult, Parser};
