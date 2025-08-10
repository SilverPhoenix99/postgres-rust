/// Alias: `FetchStmt`
pub(super) fn move_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        MOVE fetch_args
    */

    let (_, stmt) = seq!(Move, parser(|_| todo!()))
        .parse(ctx)?;

    Ok(stmt)
}

use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Move;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
