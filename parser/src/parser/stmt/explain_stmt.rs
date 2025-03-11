/// Alias: `ExplainStmt`
pub(super) fn explain_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        EXPLAIN ExplainableStmt
        EXPLAIN analyze_keyword opt_verbose ExplainableStmt
        EXPLAIN VERBOSE ExplainableStmt
        EXPLAIN '(' utility_option_list ')' ExplainableStmt
    */

    Explain
        .map(|_| todo!())
}

use crate::lexer::Keyword::Explain;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
