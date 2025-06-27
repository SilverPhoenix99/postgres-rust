/// Alias: `ReindexStmt`
pub(super) fn reindex_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        REINDEX opt_reindex_option_list reindex_target_relation opt_concurrently qualified_name
        REINDEX opt_reindex_option_list SCHEMA opt_concurrently ColId
        REINDEX opt_reindex_option_list reindex_target_all opt_concurrently opt_single_name
    */

    let (_, stmt) = seq!(stream => Reindex, parser(|_| todo!()))?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Reindex;
