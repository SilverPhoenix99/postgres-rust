impl Parser<'_> {
    pub(in crate::parser) fn set_stmt(&mut self) -> OptResult<AstNode> {

        // TODO Conflicts

        if self.buffer.consume_kw_eq(Set)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Set;
use crate::parser::AstNode;
use crate::parser::OptResult;
use crate::parser::Parser;
