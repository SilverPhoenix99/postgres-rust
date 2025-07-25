/// Alias: `ReindexStmt`
pub(super) fn reindex_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        REINDEX ( utility_options )? reindex_target_relation opt_concurrently qualified_name
        REINDEX ( utility_options )? SCHEMA opt_concurrently ColId
        REINDEX ( utility_options )? reindex_target_all opt_concurrently opt_single_name
    */

    let (_, stmt) = (Reindex, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Reindex;
