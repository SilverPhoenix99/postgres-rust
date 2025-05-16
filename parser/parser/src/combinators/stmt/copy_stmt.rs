/// Alias: `CopyStmt`
pub(super) fn copy_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        COPY opt_binary qualified_name opt_column_list copy_from opt_program copy_file_name copy_delimiter opt_with copy_options where_clause
        COPY '(' PreparableStmt ')' TO opt_program copy_file_name opt_with copy_options
    */

    CopyKw
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::RawStmt;
use postgres_parser_lexer::Keyword::CopyKw;
