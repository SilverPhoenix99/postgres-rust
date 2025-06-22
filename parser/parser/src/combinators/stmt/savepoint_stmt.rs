pub(super) fn savepoint_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
    TransactionStmt:
        SAVEPOINT ColId
    */

    Savepoint
        .and_right(parser(col_id))
        .map(TransactionStmt::Savepoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_savepoint() {
        let mut stream = TokenStream::new("savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Savepoint("test_ident".into())), savepoint_stmt().parse(&mut stream));
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::{parser, Combinator};
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Savepoint;
