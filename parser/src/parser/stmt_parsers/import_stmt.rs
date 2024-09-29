impl Parser<'_> {
    pub(in crate::parser) fn import_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Import))?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Reserved(Foreign)).required()?;
        self.buffer.consume_kw_eq(Unreserved(Schema)).required()?;

        todo!()
    }
}

use crate::lexer::Keyword::{Reserved, Unreserved};
use crate::lexer::ReservedKeyword::Foreign;
use crate::lexer::UnreservedKeyword::{Import, Schema};
use crate::parser::result::OptionalResult;
use crate::parser::{AstNode, OptResult, Parser};
