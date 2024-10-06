impl Parser<'_> {
    /// Alias: `LockStmt`
    pub(in crate::parser) fn lock_stmt(&mut self) -> OptResult<AstNode> {

        /*
            LOCK_P opt_table relation_expr_list opt_lock opt_nowait
        */

        if self.buffer.consume_kw_eq(Lock)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Lock;
use crate::parser::AstNode;
use crate::parser::OptResult;
use crate::parser::Parser;
