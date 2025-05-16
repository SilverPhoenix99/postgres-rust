/// Alias: `TruncateStmt`
pub(super) fn truncate_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        TRUNCATE opt_table relation_expr_list opt_restart_seqs opt_drop_behavior
    */

    Truncate
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Truncate;
