impl Parser<'_> {
    /// Alias: `CallStmt`
    pub(in crate::parser) fn call_stmt(&mut self) -> OptResult<AstNode> {

        /*
        CallStmt:
            CALL func_application
        */

        if self.buffer.consume_kw_eq(Call)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Call;
use crate::parser::{AstNode, OptResult, Parser};
