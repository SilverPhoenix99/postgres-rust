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
                .and_right(string())
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
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
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

use crate::lexer::Keyword::Prepared;
use crate::lexer::Keyword::Rollback;
use crate::lexer::Keyword::Savepoint;
use crate::lexer::Keyword::To;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::ast_node::TransactionStmt::RollbackPrepared;
use crate::parser::ast_node::TransactionStmt::RollbackTo;
use crate::parser::col_id;
use crate::parser::combinators::match_first;
use crate::parser::combinators::string;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::opt_transaction::opt_transaction;
use crate::parser::opt_transaction_chain;
