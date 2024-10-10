impl Parser<'_> {
    /// Alias: `LockStmt`
    pub(in crate::parser) fn lock_stmt(&mut self) -> Result<AstNode, ScanErrorKind> {

        /*
            LOCK_P opt_table relation_expr_list opt_lock opt_nowait
        */

        self.buffer.consume_kw_eq(Lock)?;

        todo!()
    }
}

use crate::lexer::Keyword::Lock;
use crate::parser::result::ScanErrorKind;
use crate::parser::AstNode;
use crate::parser::Parser;
