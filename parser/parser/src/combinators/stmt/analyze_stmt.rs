/// Alias: `AnalyzeStmt`
pub(super) fn analyze_stmt(stream: &mut TokenStream) -> Result<RawStmt> {

    /*
        (ANALYSE | ANALYZE) '(' utility_option_list ')' opt_vacuum_relation_list
        (ANALYSE | ANALYZE) (VERBOSE)? opt_vacuum_relation_list
    */

    seq!(=>
        choice!(parsed stream => Analyze, Analyse),
        parser(|_| todo!()).parse(stream)
    )
        .map(|(_, stmt)| stmt)
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Analyse;
use pg_lexer::Keyword::Analyze;
