impl Parser<'_> {
    /// Alias: `ClusterStmt`
    pub(in crate::parser) fn cluster_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            CLUSTER '(' utility_option_list ')'
            CLUSTER '(' utility_option_list ')' qualified_name cluster_index_specification
            CLUSTER opt_verbose
            CLUSTER opt_verbose name ON qualified_name
            CLUSTER opt_verbose qualified_name cluster_index_specification
        */

        todo!()
    }
}

use crate::parser::{
    ast_node::RawStmt,
    ParseResult,
    Parser
};
