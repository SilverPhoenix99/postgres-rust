/// Alias: `DoStmt`
pub(super) fn do_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        DO dostmt_opt_list
    */

    Do
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::RawStmt;
use postgres_parser_lexer::Keyword::Do;
