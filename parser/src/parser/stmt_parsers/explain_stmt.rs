impl Parser<'_> {
    /// Alias: `ExplainStmt`
    pub(in crate::parser) fn explain_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            EXPLAIN ExplainableStmt
            EXPLAIN analyze_keyword opt_verbose ExplainableStmt
            EXPLAIN VERBOSE ExplainableStmt
            EXPLAIN '(' utility_option_list ')' ExplainableStmt
        */

        todo!()
    }
}

use crate::parser::{
    ast_node::RawStmt,
    ParseResult,
    Parser
};
