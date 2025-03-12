/// Alias: `DoStmt`
pub(super) fn do_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        DO dostmt_opt_list
    */

    Do
        .map(|_| todo!())
}

use crate::lexer::Keyword::Do;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
