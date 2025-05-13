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

use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::RawStmt;
use postgres_parser_lexer::Keyword::Cluster;
