pub(super) fn rollback_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
        ROLLBACK PREPARED SCONST
        ROLLBACK opt_transaction TO SAVEPOINT ColId
        ROLLBACK opt_transaction TO ColId
        ROLLBACK opt_transaction opt_transaction_chain
    */

    Rollback.and_right(
        match_first!{
            Prepared
                .and_right(parser(string))
                .map(RollbackPrepared),
            opt_transaction().and_right(
                match_first!{
                    To.and(Savepoint.optional())
                        .and_right(col_id())
                        .map(RollbackTo),
                    opt_transaction_chain()
                        .map(|chain| TransactionStmt::Rollback { chain })
                }
            )
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("rollback", TransactionStmt::Rollback { chain: false })]
    #[test_case("rollback and chain", TransactionStmt::Rollback { chain: true })]
    #[test_case("rollback and no chain", TransactionStmt::Rollback { chain: false })]
    #[test_case("rollback to test_ident", TransactionStmt::RollbackTo("test_ident".into()))]
    #[test_case("rollback to savepoint test_ident", TransactionStmt::RollbackTo("test_ident".into()))]
    #[test_case("rollback transaction", TransactionStmt::Rollback { chain: false })]
    #[test_case("rollback transaction and chain", TransactionStmt::Rollback { chain: true })]
    #[test_case("rollback transaction and no chain", TransactionStmt::Rollback { chain: false })]
    #[test_case("rollback transaction to test_ident", TransactionStmt::RollbackTo("test_ident".into()))]
    #[test_case("rollback transaction to savepoint test_ident", TransactionStmt::RollbackTo("test_ident".into()))]
    #[test_case("rollback prepared 'test-string'", TransactionStmt::RollbackPrepared("test-string".into()))]
    fn test_rollback(source: &str, expected: TransactionStmt) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = rollback_stmt().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::opt_transaction;
use crate::combinators::opt_transaction_chain;
use pg_ast::TransactionStmt;
use pg_ast::TransactionStmt::RollbackPrepared;
use pg_ast::TransactionStmt::RollbackTo;
use pg_lexer::Keyword::Prepared;
use pg_lexer::Keyword::Rollback;
use pg_lexer::Keyword::Savepoint;
use pg_lexer::Keyword::To;
