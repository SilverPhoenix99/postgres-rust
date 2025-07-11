/// Alias: `AnalyzeStmt`
pub(super) fn analyze_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        (ANALYSE | ANALYZE) '(' utility_option_list ')' opt_vacuum_relation_list
        (ANALYSE | ANALYZE) ( VERBOSE )? opt_vacuum_relation_list
    */

    let (_, stmt) = (
        analyze_keyword,
        parser(|_| todo!())
    ).parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::stmt::analyze_keyword;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
