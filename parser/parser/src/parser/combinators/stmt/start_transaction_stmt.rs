pub(super) fn start_transaction_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
        START TRANSACTION opt_transaction_mode_list
    */

    sequence!(
        Start.and(Transaction).skip(),
        transaction_mode_list().optional()
    ).map(|(_, tx_modes)|
        TransactionStmt::Start(tx_modes.unwrap_or_default())
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use postgres_parser_ast::TransactionMode::Deferrable;
    use postgres_parser_ast::TransactionMode::ReadOnly;
    use postgres_parser_ast::TransactionMode::ReadWrite;

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

use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::transaction_mode_list;
use postgres_parser_ast::TransactionStmt;
use postgres_parser_lexer::Keyword::Start;
use postgres_parser_lexer::Keyword::Transaction;
