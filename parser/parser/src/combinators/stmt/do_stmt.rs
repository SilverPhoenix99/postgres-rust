/// Alias: `DoStmt`
pub(super) fn do_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        DO dostmt_opt_list
    */
    
    let (_, stmt) = seq!(stream => Do, parser(|_| todo!()))?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Do;
