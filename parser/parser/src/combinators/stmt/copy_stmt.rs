/// Alias: `CopyStmt`
pub(super) fn copy_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        COPY opt_binary qualified_name opt_column_list copy_from opt_program copy_file_name copy_delimiter opt_with copy_options where_clause
        COPY '(' PreparableStmt ')' TO opt_program copy_file_name opt_with copy_options
    */

    let (_, stmt) = seq!(stream => CopyKw, parser(|_| todo!()))?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::CopyKw;
