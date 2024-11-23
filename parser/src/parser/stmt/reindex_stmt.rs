/// Alias: `ReindexStmt`
pub(in crate::parser) fn reindex_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        REINDEX opt_reindex_option_list reindex_target_relation opt_concurrently qualified_name
        REINDEX opt_reindex_option_list SCHEMA opt_concurrently ColId
        REINDEX opt_reindex_option_list reindex_target_all opt_concurrently opt_single_name
    */

    keyword(Reindex)
        .map(|_| todo!())
}

use crate::lexer::Keyword::Reindex;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::keyword;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
