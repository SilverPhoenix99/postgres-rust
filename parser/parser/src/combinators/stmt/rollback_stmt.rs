pub(super) fn rollback_stmt(stream: &mut TokenStream) -> scan::Result<TransactionStmt> {

    /*
        ROLLBACK PREPARED SCONST
        ROLLBACK ( work_or_transaction )? TO SAVEPOINT ColId
        ROLLBACK ( work_or_transaction )? TO ColId
        ROLLBACK ( work_or_transaction )? ( transaction_chain )?
    */

    let (_, stmt) = seq!(
        Rollback,
        alt!(
            seq!(Prepared, string)
                .map(|(_, name)| RollbackPrepared(name)),
            seq!(
                work_or_transaction.optional(),
                alt!(
                    seq!(To, Savepoint.optional(), col_id)
                        .map(|(.., name)| RollbackTo(name)),
                    transaction_chain
                        .optional()
                        .map(Option::unwrap_or_default)
                        .map(TransactionStmt::Rollback)
                )
            )
                .map(|(_, stmt)| stmt)
        )
    ).parse(stream)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::TransactionChain::{NoChain, WithChain};
    use test_case::test_case;

    #[test_case("rollback", TransactionStmt::Rollback(NoChain))]
    #[test_case("rollback and chain", TransactionStmt::Rollback(WithChain))]
    #[test_case("rollback and no chain", TransactionStmt::Rollback(NoChain))]
    #[test_case("rollback to test_ident", TransactionStmt::RollbackTo("test_ident".into()))]
    #[test_case("rollback to savepoint test_ident", TransactionStmt::RollbackTo("test_ident".into()))]
    #[test_case("rollback transaction", TransactionStmt::Rollback(NoChain))]
    #[test_case("rollback transaction and chain", TransactionStmt::Rollback(WithChain))]
    #[test_case("rollback transaction and no chain", TransactionStmt::Rollback(NoChain))]
    #[test_case("rollback transaction to test_ident", TransactionStmt::RollbackTo("test_ident".into()))]
    #[test_case("rollback transaction to savepoint test_ident", TransactionStmt::RollbackTo("test_ident".into()))]
    #[test_case("rollback prepared 'test-string'", TransactionStmt::RollbackPrepared("test-string".into()))]
    fn test_rollback(source: &str, expected: TransactionStmt) {
        test_parser!(source, rollback_stmt, expected)
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::combinators::transaction_chain;
use crate::combinators::work_or_transaction;
use pg_ast::TransactionStmt;
use pg_ast::TransactionStmt::RollbackPrepared;
use pg_ast::TransactionStmt::RollbackTo;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Prepared;
use pg_lexer::Keyword::Rollback;
use pg_lexer::Keyword::Savepoint;
use pg_lexer::Keyword::To;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
