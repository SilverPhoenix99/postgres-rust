/// Alias: `FetchStmt`
pub(super) fn move_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        MOVE fetch_args
    */

    let (_, stmt) = (Move, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Move;
