impl Parser<'_> {
    /// Alias: `ClusterStmt`
    pub(in crate::parser) fn cluster_stmt(&mut self) -> ScanResult<AstNode> {

        /*
            CLUSTER '(' utility_option_list ')'
            CLUSTER '(' utility_option_list ')' qualified_name cluster_index_specification
            CLUSTER opt_verbose
            CLUSTER opt_verbose name ON qualified_name
            CLUSTER opt_verbose qualified_name cluster_index_specification
        */

        self.buffer.consume_kw_eq(Cluster)?;

        todo!()
    }
}

use crate::lexer::Keyword::Cluster;
use crate::parser::AstNode;
use crate::parser::Parser;
use crate::parser::result::ScanResult;
