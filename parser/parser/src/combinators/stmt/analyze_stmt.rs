/// Alias: `AnalyzeStmt`
pub(super) fn analyze_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        (ANALYSE | ANALYZE) '(' utility_option_list ')' opt_vacuum_relation_list
        (ANALYSE | ANALYZE) (VERBOSE)? opt_vacuum_relation_list
    */

    let (_, stmt) = (
        or((Analyze, Analyse)),
        parser(|_| todo!())
    ).parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Analyse;
use pg_lexer::Keyword::Analyze;
