/// Alias: `ClusterStmt`
pub(super) fn cluster_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
          CLUSTER utility_options qualified_name cluster_index_specification
        | CLUSTER ( utility_options )?
        | CLUSTER ( VERBOSE )? qualified_name cluster_index_specification
        | CLUSTER VERBOSE
        | CLUSTER ( VERBOSE )? ColId ON qualified_name
    */

    let (_, stmt) = seq!(Cluster, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Cluster;
use pg_parser_core::scan;
