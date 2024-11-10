impl Parser<'_> {
    /// Alias: `AnalyzeStmt`
    pub(in crate::parser) fn analyze_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            (ANALYSE | ANALYZE) '(' utility_option_list ')' opt_vacuum_relation_list
            (ANALYSE | ANALYZE) (VERBOSE)? opt_vacuum_relation_list
        */

        todo!()
    }
}

use crate::parser::{
    ast_node::RawStmt,
    ParseResult,
    Parser
};
