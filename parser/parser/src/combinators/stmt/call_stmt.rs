/// Alias: `CallStmt`
pub(super) fn call_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
    CallStmt:
        CALL func_application
    */

    let (_, stmt) = seq!(Call, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Call;
