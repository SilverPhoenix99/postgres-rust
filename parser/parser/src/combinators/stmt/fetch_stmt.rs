/// Alias: `FetchStmt`
pub(super) fn fetch_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        FETCH fetch_args
    */

    let (_, stmt) = seq!(Fetch, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Fetch;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
