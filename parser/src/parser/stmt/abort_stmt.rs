pub(in crate::parser) fn abort_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
    TransactionStmt:
        ABORT_P opt_transaction opt_transaction_chain
    */

    Abort
        .and(opt_transaction())
        .and_right(opt_transaction_chain())
        .map(|chain| TransactionStmt::Rollback { chain })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("abort", false)]
    #[test_case("abort and chain", true)]
    #[test_case("abort and no chain", false)]
    #[test_case("abort transaction", false)]
    #[test_case("abort transaction and chain", true)]
    #[test_case("abort transaction and no chain", false)]
    fn test_abort(source: &str, expected: bool) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: expected }), abort_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Abort;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::opt_transaction::opt_transaction;
use crate::parser::opt_transaction_chain::opt_transaction_chain;
