/// Alias: `FetchStmt`
pub(super) fn fetch_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        FETCH fetch_args
    */

    let (_, stmt) = seq!(stream => Fetch, parser(|_| todo!()))?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Fetch;
use crate::scan;
use crate::stream::TokenStream;
