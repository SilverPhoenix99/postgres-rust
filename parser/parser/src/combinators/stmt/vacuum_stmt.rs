/// Alias: `VacuumStmt`
pub(super) fn vacuum_stmt(stream: &mut TokenStream<'_>) -> scan::Result<RawStmt> {

    /*
          VACUUM opt_full opt_freeze opt_verbose opt_analyze opt_vacuum_relation_list
        | VACUUM '(' utility_option_list ')' opt_vacuum_relation_list
    */

    let (_, stmt) = seq!(Vacuum, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Vacuum;
