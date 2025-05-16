pub(super) fn commit_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
        COMMIT PREPARED SCONST
        COMMIT opt_transaction opt_transaction_chain
    */

    Commit.and_right(match_first!{
            Prepared
                .and_right(string())
                .map(CommitPrepared),
            opt_transaction()
                .and_right(opt_transaction_chain())
                .map(|chain| TransactionStmt::Commit { chain })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("commit", false)]
    #[test_case("commit and chain", true)]
    #[test_case("commit and no chain", false)]
    #[test_case("commit transaction", false)]
    #[test_case("commit transaction and chain", true)]
    #[test_case("commit transaction and no chain", false)]
    fn test_commit(source: &str, expected: bool) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: expected }), commit_stmt().parse(&mut stream));
    }

    #[test]
    fn test_commit_prepared() {
        let mut stream = TokenStream::new("commit prepared 'test-name'", DEFAULT_CONFIG);
        assert_eq!(Ok(CommitPrepared("test-name".into())), commit_stmt().parse(&mut stream));
    }
}

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::opt_transaction;
use crate::combinators::opt_transaction_chain;
use pg_ast::TransactionStmt;
use pg_ast::TransactionStmt::CommitPrepared;
use pg_lexer::Keyword::Commit;
use pg_lexer::Keyword::Prepared;
