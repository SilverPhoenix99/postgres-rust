pub(super) fn commit_stmt(stream: &mut TokenStream) -> scan::Result<TransactionStmt> {

    /*
        COMMIT PREPARED SCONST
        COMMIT opt_transaction opt_transaction_chain
    */

    let (_, stmt) = seq!(=>
        Commit.parse(stream),
        choice!(stream =>
            seq!(stream => Prepared, string)
                .map(|(_, tx_name)| CommitPrepared(tx_name)),
            seq!(stream => opt_transaction, opt_transaction_chain)
                .map(|(_, chain)| TransactionStmt::Commit { chain })
        )
    )?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("commit", false)]
    #[test_case("commit and chain", true)]
    #[test_case("commit and no chain", false)]
    #[test_case("commit transaction", false)]
    #[test_case("commit transaction and chain", true)]
    #[test_case("commit transaction and no chain", false)]
    fn test_commit(source: &str, expected: bool) {
        test_parser!(source, commit_stmt, TransactionStmt::Commit { chain: expected })
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

use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::opt_transaction;
use crate::combinators::opt_transaction_chain;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::TransactionStmt;
use pg_ast::TransactionStmt::CommitPrepared;
use pg_lexer::Keyword::Commit;
use pg_lexer::Keyword::Prepared;
