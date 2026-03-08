/// Alias: `FetchStmt`
pub(super) fn move_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        MOVE fetch_args
    */

    let (_, stmt) = seq!(Move, parser(|_| todo!()))
        .parse(ctx)?;

    Ok(stmt)
}

use crate::combinators::core::parser;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Move;
use pg_parser_core::scan;
