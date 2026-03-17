/// Alias: `RepackStmt`
pub(super) fn repack_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          REPACK opt_utility_option_list vacuum_relation USING INDEX ColId
        | REPACK opt_utility_option_list vacuum_relation opt_usingindex
        | REPACK opt_utility_option_list opt_usingindex
        | CLUSTER utility_options qualified_name cluster_index_specification
        | CLUSTER ( utility_options )?
        | CLUSTER ( VERBOSE )? qualified_name cluster_index_specification
        | CLUSTER VERBOSE
        | CLUSTER ( VERBOSE )? ColId ON qualified_name
    */

    let (_, stmt) = alt!(
        seq!(Repack, parser(|_| todo!())),
        seq!(Cluster, parser(|_| todo!()))
    ).parse(ctx)?;

    Ok(stmt)
}

use crate::alt;
use crate::combinators::core::parser;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Cluster;
use pg_lexer::Keyword::Repack;
use pg_parser_core::scan;
