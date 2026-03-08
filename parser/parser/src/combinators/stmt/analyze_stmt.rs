/// Alias: `AnalyzeStmt`
pub(super) fn analyze_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          (ANALYSE | ANALYZE) ( utility_options )? opt_vacuum_relation_list
        | (ANALYSE | ANALYZE) VERBOSE opt_vacuum_relation_list
    */

    let (_, stmt) = seq!(
        analyze_keyword,
        parser(|_| todo!())
    ).parse(ctx)?;

    Ok(stmt)
}

use crate::combinators::analyze_keyword;
use crate::combinators::core::parser;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::RawStmt;
use pg_parser_core::scan;
