/// Alias: `FetchStmt`
pub(super) fn fetch_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        FETCH fetch_args
    */

    Fetch
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Fetch;
