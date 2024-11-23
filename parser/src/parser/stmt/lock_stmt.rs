/// Alias: `LockStmt`
pub(in crate::parser) fn lock_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        LOCK_P opt_table relation_expr_list opt_lock opt_nowait
    */

    keyword(Load)
        .map(|_| todo!())
}

use crate::lexer::Keyword::Load;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::keyword;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
