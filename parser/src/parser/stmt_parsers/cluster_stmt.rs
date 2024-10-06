impl Parser<'_> {
    /// Alias: `ClusterStmt`
    pub(in crate::parser) fn cluster_stmt(&mut self) -> OptResult<AstNode> {

        /*
            CLUSTER '(' utility_option_list ')'
            CLUSTER '(' utility_option_list ')' qualified_name cluster_index_specification
            CLUSTER opt_verbose
            CLUSTER opt_verbose name ON qualified_name
            CLUSTER opt_verbose qualified_name cluster_index_specification
        */

        if self.buffer.consume_kw_eq(Cluster)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Cluster;
use crate::parser::AstNode;
use crate::parser::OptResult;
use crate::parser::Parser;
