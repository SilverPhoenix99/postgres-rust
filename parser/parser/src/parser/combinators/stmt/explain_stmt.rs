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

use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::Explain;
