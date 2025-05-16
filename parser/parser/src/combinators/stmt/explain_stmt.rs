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

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Explain;
