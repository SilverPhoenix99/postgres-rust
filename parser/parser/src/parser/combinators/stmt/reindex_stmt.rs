/// Alias: `ReindexStmt`
pub(super) fn reindex_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        REINDEX opt_reindex_option_list reindex_target_relation opt_concurrently qualified_name
        REINDEX opt_reindex_option_list SCHEMA opt_concurrently ColId
        REINDEX opt_reindex_option_list reindex_target_all opt_concurrently opt_single_name
    */

    Reindex
        .map(|_| todo!())
}

use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::Reindex;
