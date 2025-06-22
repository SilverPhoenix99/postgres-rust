/// Alias: `DoStmt`
pub(super) fn do_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        DO dostmt_opt_list
    */

    Do
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Do;
