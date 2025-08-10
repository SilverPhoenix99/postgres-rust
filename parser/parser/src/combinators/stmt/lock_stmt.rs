/// Alias: `LockStmt`
pub(super) fn lock_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        LOCK_P opt_table relation_expr_list opt_lock opt_nowait
    */

    let (_, stmt) = seq!(Lock, parser(|_| todo!()))
        .parse(ctx)?;

    Ok(stmt)
}

use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Lock;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
