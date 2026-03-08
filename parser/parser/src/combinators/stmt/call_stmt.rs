/// Alias: `CallStmt`
pub(super) fn call_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
    CallStmt:
        CALL func_application
    */

    let (_, stmt) = seq!(Call, parser(|_| todo!()))
        .parse(ctx)?;

    Ok(stmt)
}

use crate::combinators::core::parser;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Call;
use pg_parser_core::scan;
