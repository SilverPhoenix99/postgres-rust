/// Alias: `CopyStmt`
pub(super) fn copy_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
          COPY opt_binary qualified_name opt_column_list copy_from opt_program copy_file_name copy_delimiter opt_with copy_options where_clause
        | COPY '(' PreparableStmt ')' TO opt_program copy_file_name opt_with copy_options
    */

    let (_, stmt) = seq!(CopyKw, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use pg_ast::RawStmt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::CopyKw;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
