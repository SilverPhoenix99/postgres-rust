pub(super) fn abort_stmt(stream: &mut TokenStream) -> Result<TransactionStmt> {

    /*
    TransactionStmt:
        ABORT_P opt_transaction opt_transaction_chain
    */

    seq!(stream =>
        Abort,
        opt_transaction,
        opt_transaction_chain
    )
        .map(|(.., chain)| TransactionStmt::Rollback { chain })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("abort", false)]
    #[test_case("abort and chain", true)]
    #[test_case("abort and no chain", false)]
    #[test_case("abort transaction", false)]
    #[test_case("abort transaction and chain", true)]
    #[test_case("abort transaction and no chain", false)]
    fn test_abort(source: &str, expected: bool) {
        test_parser!(source, abort_stmt, TransactionStmt::Rollback { chain: expected })
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::opt_transaction;
use crate::combinators::opt_transaction_chain;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Abort;
