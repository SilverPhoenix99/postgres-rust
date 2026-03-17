/// Alias: `WaitStmt`
pub(super) fn wait_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          WAIT FOR LSN SCONST ( wait_with_clause )?
    */

    let (_, _, stmt) = seq!(Wait, For, parser(|_| todo!()))
        .parse(ctx)?;

    Ok(stmt)
}

/// Alias: `opt_wait_with_clause`
fn wait_with_clause(ctx: &mut ParserContext) -> scan::Result<Vec<UtilityOption>> {

    /*
          WITH '(' utility_option_list ')'
    */

    todo!()
}

use crate::combinators::core::parser;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::RawStmt;
use pg_ast::UtilityOption;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Wait;
use pg_parser_core::scan;
