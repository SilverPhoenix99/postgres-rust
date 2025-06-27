/// Alias: `LockStmt`
pub(super) fn lock_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        LOCK_P opt_table relation_expr_list opt_lock opt_nowait
    */

    let (_, stmt) = seq!(stream => Lock, parser(|_| todo!()))?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Lock;
