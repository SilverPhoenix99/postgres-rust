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

use crate::combinators::foundation::seq;
use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Call;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
