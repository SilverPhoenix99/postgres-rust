/// Alias: `VacuumStmt`
pub(super) fn vacuum_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          VACUUM opt_full opt_freeze opt_verbose opt_analyze opt_vacuum_relation_list
        | VACUUM '(' utility_option_list ')' opt_vacuum_relation_list
    */

    let (_, stmt) = seq!(Vacuum, parser(|_| todo!()))
        .parse(ctx)?;

    Ok(stmt)
}

use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Vacuum;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
