/// Alias: `FetchStmt`
pub(super) fn fetch_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        FETCH fetch_args
    */

    let (_, stmt) = seq!(Fetch, parser(|_| todo!()))
        .parse(ctx)?;

    Ok(stmt)
}

use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Fetch;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
