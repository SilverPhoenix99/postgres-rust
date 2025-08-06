/// Alias: `ExplainStmt`
pub(super) fn explain_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
          EXPLAIN ExplainableStmt
        | EXPLAIN analyze_keyword opt_verbose ExplainableStmt
        | EXPLAIN VERBOSE ExplainableStmt
        | EXPLAIN '(' utility_option_list ')' ExplainableStmt
    */

    let (_, stmt) = seq!(Explain, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Explain;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
