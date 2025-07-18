/// Alias: `DoStmt`
pub(super) fn do_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        DO dostmt_opt_list
    */
    
    let (_, stmt) = (Do, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Do;
