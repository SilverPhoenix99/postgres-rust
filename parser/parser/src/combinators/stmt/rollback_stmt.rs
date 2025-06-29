pub(super) fn rollback_stmt(stream: &mut TokenStream) -> scan::Result<TransactionStmt> {

    /*
        ROLLBACK PREPARED SCONST
        ROLLBACK opt_transaction TO SAVEPOINT ColId
        ROLLBACK opt_transaction TO ColId
        ROLLBACK opt_transaction opt_transaction_chain
    */

    let (_, stmt) = (
        Rollback,
        or((
            (Prepared, string)
                .map(|(_, name)| RollbackPrepared(name)),
            (
                opt_transaction,
                or((
                    ( To, Savepoint.optional(), col_id)
                        .map(|(.., name)| RollbackTo(name)),
                    opt_transaction_chain
                        .map(|chain| TransactionStmt::Rollback { chain })
                ))
            )
                .map(|(_, stmt)| stmt)
        ))
    ).parse(stream)?;
    
    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
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
        test_parser!(source, rollback_stmt, expected)
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::or;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::opt_transaction;
use crate::combinators::opt_transaction_chain;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::TransactionStmt;
use pg_ast::TransactionStmt::RollbackPrepared;
use pg_ast::TransactionStmt::RollbackTo;
use pg_lexer::Keyword::Prepared;
use pg_lexer::Keyword::Rollback;
use pg_lexer::Keyword::Savepoint;
use pg_lexer::Keyword::To;
