pub(super) fn abort_stmt(stream: &mut TokenStream) -> scan::Result<TransactionStmt> {

    /*
    TransactionStmt:
        ABORT_P ( work_or_transaction )? ( transaction_chain )?
    */

    let (.., chain) = (
        Abort,
        work_or_transaction.optional(),
        transaction_chain
            .optional()
            .map(Option::unwrap_or_default)
    ).parse(stream)?;

    Ok(TransactionStmt::Rollback(chain))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::TransactionChain;
    use test_case::test_case;

    #[test_case("abort", TransactionChain::NoChain)]
    #[test_case("abort and chain", TransactionChain::WithChain)]
    #[test_case("abort and no chain", TransactionChain::NoChain)]
    #[test_case("abort transaction", TransactionChain::NoChain)]
    #[test_case("abort transaction and chain", TransactionChain::WithChain)]
    #[test_case("abort transaction and no chain", TransactionChain::NoChain)]
    fn test_abort(source: &str, expected: TransactionChain) {
        test_parser!(source, abort_stmt, TransactionStmt::Rollback(expected))
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::transaction_chain;
use crate::combinators::work_or_transaction;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Abort;
