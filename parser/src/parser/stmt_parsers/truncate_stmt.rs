impl Parser<'_> {
    /// Alias: `TruncateStmt`
    pub(in crate::parser) fn truncate_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            TRUNCATE opt_table relation_expr_list opt_restart_seqs opt_drop_behavior
        */

        todo!()
    }
}

use crate::parser::{
    ast_node::RawStmt,
    ParseResult,
    Parser
};
