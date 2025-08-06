/// Alias: `AnalyzeStmt`
pub(super) fn analyze_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
          (ANALYSE | ANALYZE) ( utility_options )? opt_vacuum_relation_list
        | (ANALYSE | ANALYZE) VERBOSE opt_vacuum_relation_list
    */

    let (_, stmt) = seq!(
        analyze_keyword,
        parser(|_| todo!())
    ).parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::stmt::analyze_keyword;
use pg_ast::RawStmt;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
