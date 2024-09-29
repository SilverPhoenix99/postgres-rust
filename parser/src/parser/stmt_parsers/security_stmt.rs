impl Parser<'_> {
    pub(in crate::parser) fn security_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Security))?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Unreserved(Label)).required()?;

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::{Label, Security};
use crate::parser::result::OptionalResult;
use crate::parser::{AstNode, OptResult, Parser};
