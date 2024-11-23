/// Alias: `AnalyzeStmt`
pub(in crate::parser) fn analyze_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        (ANALYSE | ANALYZE) '(' utility_option_list ')' opt_vacuum_relation_list
        (ANALYSE | ANALYZE) (VERBOSE)? opt_vacuum_relation_list
    */

    keyword(Analyze).or(keyword(Analyse))
        .map(|_| todo!())
}

use crate::lexer::Keyword::{Analyse, Analyze};
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::{keyword, Combinator, CombinatorHelpers};
