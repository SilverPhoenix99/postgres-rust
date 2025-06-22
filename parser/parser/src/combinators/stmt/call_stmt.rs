/// Alias: `CallStmt`
pub(super) fn call_stmt() -> impl Combinator<Output = RawStmt> {

    /*
    CallStmt:
        CALL func_application
    */

    Call
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Call;
