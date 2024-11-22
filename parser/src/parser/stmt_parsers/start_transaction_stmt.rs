pub(in crate::parser) fn start_transaction_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
        START TRANSACTION opt_transaction_mode_list
    */

    keyword(Start)
        .and(keyword(Transaction))
        .and_right(
            transaction_mode_list::transaction_mode_list()
                .optional()
                .map(Option::unwrap_or_default)
                .map(TransactionStmt::Start)
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::TransactionMode::{Deferrable, ReadOnly, ReadWrite};
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

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

use crate::lexer::Keyword::{Start, Transaction};
use crate::parser::ast_node::TransactionStmt;
use crate::parser::combinators::{keyword, Combinator, CombinatorHelpers};
use crate::parser::transaction_mode_list;
