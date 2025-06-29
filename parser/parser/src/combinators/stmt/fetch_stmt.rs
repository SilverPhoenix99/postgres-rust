/// Alias: `FetchStmt`
pub(super) fn fetch_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        FETCH fetch_args
    */

    let (_, stmt) = (Fetch, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Fetch;
