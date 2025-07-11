/// Alias: `CheckPointStmt`
pub(super) fn check_point_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
          CHECKPOINT
        | CHECKPOINT '(' utility_option_list ')'
    */

    Checkpoint
        .map(|_| todo!())
        .parse(stream)
}

use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Checkpoint;
