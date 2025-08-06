pub(super) fn commit_stmt(stream: &mut TokenStream) -> scan::Result<TransactionStmt> {

    /*
        COMMIT PREPARED SCONST
        COMMIT ( work_or_transaction )? ( transaction_chain )?
    */

    let (_, stmt) = seq!(
        Commit,
        alt!(
            seq!(Prepared, string)
                .map(|(_, tx_name)| CommitPrepared(tx_name)),
            seq!(
                work_or_transaction.optional(),
                transaction_chain
                    .optional()
                    .map(Option::unwrap_or_default)
            ).map(|(_, chain)| TransactionStmt::Commit(chain))
        )
    ).parse(stream)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::TransactionChain;
    use test_case::test_case;

    #[test_case("commit", TransactionChain::NoChain)]
    #[test_case("commit and chain", TransactionChain::WithChain)]
    #[test_case("commit and no chain", TransactionChain::NoChain)]
    #[test_case("commit transaction", TransactionChain::NoChain)]
    #[test_case("commit transaction and chain", TransactionChain::WithChain)]
    #[test_case("commit transaction and no chain", TransactionChain::NoChain)]
    fn test_commit(source: &str, expected: TransactionChain) {
        test_parser!(source, commit_stmt, TransactionStmt::Commit(expected))
    }

    #[test]
    fn test_commit_prepared() {
        test_parser!(
            source = "commit prepared 'test-name'",
            parser = commit_stmt,
            expected = CommitPrepared("test-name".into())
        )
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::transaction_chain;
use crate::combinators::work_or_transaction;
use pg_ast::TransactionStmt;
use pg_ast::TransactionStmt::CommitPrepared;
use pg_lexer::Keyword::Commit;
use pg_lexer::Keyword::Prepared;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
