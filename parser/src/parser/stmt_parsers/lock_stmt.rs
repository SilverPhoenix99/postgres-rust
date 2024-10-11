impl Parser<'_> {
    /// Alias: `LockStmt`
    pub(in crate::parser) fn lock_stmt(&mut self) -> ScanResult<RawStmt> {

        /*
            LOCK_P opt_table relation_expr_list opt_lock opt_nowait
        */

        self.buffer.consume_kw_eq(Lock)?;

        todo!()
    }
}

use crate::lexer::Keyword::Lock;
use crate::parser::ast_node::RawStmt;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
