/// Alias: `AnalyzeStmt`
pub(super) fn analyze_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        (ANALYSE | ANALYZE) '(' utility_option_list ')' opt_vacuum_relation_list
        (ANALYSE | ANALYZE) (VERBOSE)? opt_vacuum_relation_list
    */

    Analyze.or(Analyse)
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Analyse;
use pg_lexer::Keyword::Analyze;
