/// Alias: `CallStmt`
pub(super) fn call_stmt() -> impl Combinator<Output = RawStmt> {

    /*
    CallStmt:
        CALL func_application
    */

    Call
        .map(|_| todo!())
}

use crate::lexer::Keyword::Call;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
