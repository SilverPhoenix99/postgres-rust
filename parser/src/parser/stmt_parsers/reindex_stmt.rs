impl Parser<'_> {
    /// Alias: `ReindexStmt`
    pub(in crate::parser) fn reindex_stmt(&mut self) -> OptResult<AstNode> {

        /*
            REINDEX opt_reindex_option_list reindex_target_relation opt_concurrently qualified_name
            REINDEX opt_reindex_option_list SCHEMA opt_concurrently ColId
            REINDEX opt_reindex_option_list reindex_target_all opt_concurrently opt_single_name
        */

        if self.buffer.consume_kw_eq(Reindex)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Reindex;
use crate::parser::{AstNode, OptResult, Parser};
