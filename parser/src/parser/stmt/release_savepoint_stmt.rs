pub(super) fn release_savepoint_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
    TransactionStmt:
        RELEASE SAVEPOINT ColId
        RELEASE ColId
    */

    Release
        .and(Savepoint.optional())
        .and_right(col_id())
        .map(TransactionStmt::Release)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_release() {
        let mut stream = TokenStream::new("release test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Release("test_ident".into())), release_savepoint_stmt().parse(&mut stream));
    }

    #[test]
    fn test_release_savepoint() {
        let mut stream = TokenStream::new("release savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Release("test_ident".into())), release_savepoint_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Release;
use crate::lexer::Keyword::Savepoint;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::col_id;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
