pub(super) fn start_transaction_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
        START TRANSACTION opt_transaction_mode_list
    */

    (
        Start.and(Transaction).skip(),
        transaction_mode_list.optional()
    ).map(|(_, tx_modes)|
        TransactionStmt::Start(tx_modes.unwrap_or_default())
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::TransactionMode::Deferrable;
    use pg_ast::TransactionMode::ReadOnly;
    use pg_ast::TransactionMode::ReadWrite;

    #[test]
    fn test_start_transaction() {
        let mut stream = TokenStream::new("start transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Start(Vec::new())), start_transaction_stmt().parse(&mut stream));
    }

    #[test]
    fn test_start_transaction_with_transaction_modes() {
        let mut stream = TokenStream::new("start transaction read only, read write deferrable", DEFAULT_CONFIG);
        let modes = vec![ReadOnly, ReadWrite, Deferrable];
        assert_eq!(Ok(TransactionStmt::Start(modes)), start_transaction_stmt().parse(&mut stream));
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::transaction_mode_list;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Start;
use pg_lexer::Keyword::Transaction;
