/// Alias: `ClusterStmt`
pub(super) fn cluster_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        CLUSTER '(' utility_option_list ')'
        CLUSTER '(' utility_option_list ')' qualified_name cluster_index_specification
        CLUSTER opt_verbose
        CLUSTER opt_verbose name ON qualified_name
        CLUSTER opt_verbose qualified_name cluster_index_specification
    */

    Cluster
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Cluster;
