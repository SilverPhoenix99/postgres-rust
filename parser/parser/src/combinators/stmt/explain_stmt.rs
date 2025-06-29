/// Alias: `ExplainStmt`
pub(super) fn explain_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        EXPLAIN ExplainableStmt
        EXPLAIN analyze_keyword opt_verbose ExplainableStmt
        EXPLAIN VERBOSE ExplainableStmt
        EXPLAIN '(' utility_option_list ')' ExplainableStmt
    */

    let (_, stmt) = (Explain, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Explain;
