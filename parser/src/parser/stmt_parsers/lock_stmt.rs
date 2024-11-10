impl Parser<'_> {
    /// Alias: `LockStmt`
    pub(in crate::parser) fn lock_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            LOCK_P opt_table relation_expr_list opt_lock opt_nowait
        */

        todo!()
    }
}

use crate::parser::{
    ast_node::RawStmt,
    ParseResult,
    Parser
};
