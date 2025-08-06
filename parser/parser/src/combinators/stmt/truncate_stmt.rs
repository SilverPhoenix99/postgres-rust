/// Alias: `TruncateStmt`
pub(super) fn truncate_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        TRUNCATE opt_table relation_expr_list opt_restart_seqs opt_drop_behavior
    */

    let (_, stmt) = seq!(Truncate, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::seq;
use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Truncate;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
