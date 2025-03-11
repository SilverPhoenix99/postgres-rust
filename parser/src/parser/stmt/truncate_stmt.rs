/// Alias: `TruncateStmt`
pub(super) fn truncate_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        TRUNCATE opt_table relation_expr_list opt_restart_seqs opt_drop_behavior
    */

    Truncate
        .map(|_| todo!())
}

use crate::lexer::Keyword::Truncate;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
