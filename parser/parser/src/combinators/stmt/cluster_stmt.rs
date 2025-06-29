/// Alias: `ClusterStmt`
pub(super) fn cluster_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        CLUSTER '(' utility_option_list ')'
        CLUSTER '(' utility_option_list ')' qualified_name cluster_index_specification
        CLUSTER opt_verbose
        CLUSTER opt_verbose name ON qualified_name
        CLUSTER opt_verbose qualified_name cluster_index_specification
    */

    let (_, stmt) = (Cluster, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Cluster;
