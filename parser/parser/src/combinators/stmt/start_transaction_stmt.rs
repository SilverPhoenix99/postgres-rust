pub(super) fn start_transaction_stmt(stream: &mut TokenStream) -> scan::Result<TransactionStmt> {

    /*
        START TRANSACTION ( transaction_mode_list )?
    */

    let (.., tx_modes) = seq!(
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
    use pg_combinators::test_parser;
    use pg_transaction_mode_ast::TransactionMode::Deferrable;
    use pg_transaction_mode_ast::TransactionMode::ReadOnly;
    use pg_transaction_mode_ast::TransactionMode::ReadWrite;

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

use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Start;
use pg_lexer::Keyword::Transaction;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_transaction_mode_ast::TransactionStmt;
use pg_transaction_mode_combinators::transaction_mode_list;
