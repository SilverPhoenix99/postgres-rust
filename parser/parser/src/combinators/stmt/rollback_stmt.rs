pub(super) fn rollback_stmt(ctx: &mut ParserContext) -> scan::Result<TransactionStmt> {

    /*
        ROLLBACK PREPARED SCONST
        ROLLBACK ( work_or_transaction )? TO SAVEPOINT ColId
        ROLLBACK ( work_or_transaction )? TO ColId
        ROLLBACK ( work_or_transaction )? ( transaction_chain )?
    */

    let (_, stmt) = seq!(
        Kw::Rollback,
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
                        .map(|chain| Rollback { chain: chain.unwrap_or_default() })
                )
            )
                .map(|(_, stmt)| stmt)
        )
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("rollback" => Ok(Rollback { chain: false }))]
    #[test_case("rollback and chain" => Ok(Rollback { chain: true }))]
    #[test_case("rollback and no chain" => Ok(Rollback { chain: false }))]
    #[test_case("rollback to test_ident" => Ok(RollbackTo("test_ident".into())))]
    #[test_case("rollback to savepoint test_ident" => Ok(RollbackTo("test_ident".into())))]
    #[test_case("rollback transaction" => Ok(Rollback { chain: false }))]
    #[test_case("rollback transaction and chain" => Ok(Rollback { chain: true }))]
    #[test_case("rollback transaction and no chain" => Ok(Rollback { chain: false }))]
    #[test_case("rollback transaction to test_ident" => Ok(RollbackTo("test_ident".into())))]
    #[test_case("rollback transaction to savepoint test_ident" => Ok(RollbackTo("test_ident".into())))]
    #[test_case("rollback prepared 'test-string'" => Ok(RollbackPrepared("test-string".into())))]
    fn test_rollback(source: &str) -> scan::Result<TransactionStmt> {
        test_parser!(source, rollback_stmt)
    }
}

use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Prepared;
use pg_lexer::Keyword::Savepoint;
use pg_lexer::Keyword::To;
use pg_parser_core::scan;
use pg_sink_combinators::col_id;
use pg_sink_combinators::transaction_chain;
use pg_sink_combinators::work_or_transaction;
use pg_transaction_stmt_ast::TransactionStmt;
use pg_transaction_stmt_ast::TransactionStmt::Rollback;
use pg_transaction_stmt_ast::TransactionStmt::RollbackPrepared;
use pg_transaction_stmt_ast::TransactionStmt::RollbackTo;
