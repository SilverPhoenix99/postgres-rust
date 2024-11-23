pub(in crate::parser) fn set_stmt() -> impl Combinator<Output = RawStmt> {

    // TODO Conflicts

    keyword(Set)
        .map(|_| todo!())
}

use crate::lexer::Keyword::Set;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::keyword;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
