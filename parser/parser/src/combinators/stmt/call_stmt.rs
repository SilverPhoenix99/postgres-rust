/// Alias: `CallStmt`
pub(super) fn call_stmt(stream: &mut TokenStream) -> Result<RawStmt> {

    /*
    CallStmt:
        CALL func_application
    */

    let (_, stmt) = seq!(stream =>
        Call,
        parser(|_| todo!())
    )?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Call;
