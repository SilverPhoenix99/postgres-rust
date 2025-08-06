/// Alias: `DoStmt`
pub(super) fn do_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        DO dostmt_opt_list
    */

    let (_, stmt) = seq!(Do, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::seq;
use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Do;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
