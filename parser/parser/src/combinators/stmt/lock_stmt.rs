/// Alias: `LockStmt`
pub(super) fn lock_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        LOCK_P opt_table relation_expr_list opt_lock opt_nowait
    */

    Lock
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Lock;
