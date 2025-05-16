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
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::RawStmt;
use postgres_parser_lexer::Keyword::Call;
