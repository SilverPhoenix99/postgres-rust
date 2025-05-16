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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

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

use crate::combinators::col_id;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::TransactionStmt;
use postgres_parser_lexer::Keyword::Release;
use postgres_parser_lexer::Keyword::Savepoint;
