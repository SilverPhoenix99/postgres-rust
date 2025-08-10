/// Alias: `DoStmt`
pub(super) fn do_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        DO dostmt_opt_list
    */

    let (_, stmt) = seq!(Do, parser(|_| todo!()))
        .parse(ctx)?;

    Ok(stmt)
}

use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Do;
use pg_parser_core::scan;
