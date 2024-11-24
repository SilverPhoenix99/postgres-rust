/// Alias: `AnalyzeStmt`
pub(in crate::parser) fn analyze_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        (ANALYSE | ANALYZE) '(' utility_option_list ')' opt_vacuum_relation_list
        (ANALYSE | ANALYZE) (VERBOSE)? opt_vacuum_relation_list
    */

    Analyze.or(Analyse)
        .map(|_| todo!())
}

use crate::lexer::Keyword::Analyse;
use crate::lexer::Keyword::Analyze;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
