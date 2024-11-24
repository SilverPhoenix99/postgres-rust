/// Alias: `CallStmt`
pub(in crate::parser) fn call_stmt() -> impl Combinator<Output = RawStmt> {

    /*
    CallStmt:
        CALL func_application
    */

    Call
        .map(|_| todo!())
}

use crate::lexer::Keyword::Call;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
