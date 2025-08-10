/// Alias: `ReindexStmt`
pub(super) fn reindex_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          REINDEX ( utility_options )? reindex_target_relation opt_concurrently qualified_name
        | REINDEX ( utility_options )? SCHEMA opt_concurrently ColId
        | REINDEX ( utility_options )? reindex_target_all opt_concurrently opt_single_name
    */

    let (_, stmt) = seq!(Reindex, parser(|_| todo!()))
        .parse(ctx)?;

    Ok(stmt)
}

use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Reindex;
use pg_parser_core::scan;
