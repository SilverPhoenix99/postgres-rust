pub(super) fn start_transaction_stmt(stream: &mut TokenStream) -> scan::Result<TransactionStmt> {

    /*
        START TRANSACTION opt_transaction_mode_list
    */

    let (.., tx_modes) = (
        Start,
        Transaction,
        transaction_mode_list.optional()
    ).parse(stream)?;

    let stmt = TransactionStmt::Start(tx_modes.unwrap_or_default());

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::TransactionMode::Deferrable;
    use pg_ast::TransactionMode::ReadOnly;
    use pg_ast::TransactionMode::ReadWrite;

    #[test]
    fn test_start_transaction() {
        test_parser!(
            source = "start transaction",
            parser = start_transaction_stmt,
            expected = TransactionStmt::Start(Vec::new())
        )
    }

    #[test]
    fn test_start_transaction_with_transaction_modes() {
        test_parser!(
            source = "start transaction read only, read write deferrable",
            parser = start_transaction_stmt,
            expected = TransactionStmt::Start(vec![ReadOnly, ReadWrite, Deferrable])
        )
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::transaction_mode_list;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Start;
use pg_lexer::Keyword::Transaction;
