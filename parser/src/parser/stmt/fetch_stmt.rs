/// Alias: `FetchStmt`
pub(in crate::parser) fn fetch_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        FETCH fetch_args
    */

    keyword(Fetch)
        .map(|_| todo!())
}

use crate::lexer::Keyword::Fetch;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::keyword;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
