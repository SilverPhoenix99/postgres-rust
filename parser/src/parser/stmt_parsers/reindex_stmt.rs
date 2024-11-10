impl Parser<'_> {
    /// Alias: `ReindexStmt`
    pub(in crate::parser) fn reindex_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            REINDEX opt_reindex_option_list reindex_target_relation opt_concurrently qualified_name
            REINDEX opt_reindex_option_list SCHEMA opt_concurrently ColId
            REINDEX opt_reindex_option_list reindex_target_all opt_concurrently opt_single_name
        */

        todo!()
    }
}

use crate::parser::{
    ast_node::RawStmt,
    ParseResult,
    Parser
};
