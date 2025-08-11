pub(crate) fn commit_stmt(ctx: &mut ParserContext) -> scan::Result<TransactionStmt> {

    /*
        COMMIT PREPARED SCONST
        COMMIT ( work_or_transaction )? ( transaction_chain )?
    */

    let (_, stmt) = seq!(
        Kw::Commit,
        alt!(
            seq!(Prepared, string)
                .map(|(_, tx_name)| CommitPrepared(tx_name)),
            seq!(
                work_or_transaction.optional(),
                transaction_chain
                    .optional()
            ).map(|(_, chain)| Commit { chain: chain.unwrap_or_default() })
        )
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("commit" => Ok(Commit { chain: false }))]
    #[test_case("commit and chain" => Ok(Commit { chain: true }))]
    #[test_case("commit and no chain" => Ok(Commit { chain: false }))]
    #[test_case("commit transaction" => Ok(Commit { chain: false }))]
    #[test_case("commit transaction and chain" => Ok(Commit { chain: true }))]
    #[test_case("commit transaction and no chain" => Ok(Commit { chain: false }))]
    fn test_commit(source: &str) -> scan::Result<TransactionStmt> {
        test_parser!(source, commit_stmt)
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

use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Prepared;
use pg_parser_core::scan;
use pg_sink_combinators::transaction_chain;
use pg_sink_combinators::work_or_transaction;
use pg_transaction_stmt_ast::TransactionStmt;
use pg_transaction_stmt_ast::TransactionStmt::Commit;
use pg_transaction_stmt_ast::TransactionStmt::CommitPrepared;
