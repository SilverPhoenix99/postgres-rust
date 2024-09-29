impl Parser<'_> {
    pub(in crate::parser) fn comment_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Comment))?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Reserved(On)).required()?;

        todo!()
    }
}

use crate::lexer::Keyword::{Reserved, Unreserved};
use crate::lexer::ReservedKeyword::On;
use crate::lexer::UnreservedKeyword::Comment;
use crate::parser::result::OptionalResult;
use crate::parser::{AstNode, OptResult, Parser};
