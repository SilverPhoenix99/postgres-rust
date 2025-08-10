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

use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
use pg_sink_combinators::analyze_keyword;
