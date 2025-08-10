/// Alias: `ClusterStmt`
pub(super) fn cluster_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          CLUSTER utility_options qualified_name cluster_index_specification
        | CLUSTER ( utility_options )?
        | CLUSTER ( VERBOSE )? qualified_name cluster_index_specification
        | CLUSTER VERBOSE
        | CLUSTER ( VERBOSE )? ColId ON qualified_name
    */

    let (_, stmt) = seq!(Cluster, parser(|_| todo!()))
        .parse(ctx)?;

    Ok(stmt)
}

use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Cluster;
use pg_parser_core::scan;
