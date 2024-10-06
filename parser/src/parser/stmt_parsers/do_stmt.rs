impl Parser<'_> {
    /// Alias: `DoStmt`
    pub(in crate::parser) fn do_stmt(&mut self) -> OptResult<AstNode> {

        /*
            DO dostmt_opt_list
        */

        if self.buffer.consume_kw_eq(Do)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Do;
use crate::parser::{AstNode, OptResult, Parser};
