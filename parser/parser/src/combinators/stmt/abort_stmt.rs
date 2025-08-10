pub(super) fn abort_stmt(ctx: &mut ParserContext) -> scan::Result<TransactionStmt> {

    /*
    TransactionStmt:
        ABORT_P ( work_or_transaction )? ( transaction_chain )?
    */

    let (.., chain) = seq!(
        Abort,
        work_or_transaction.optional(),
        transaction_chain
            .optional()
    ).parse(ctx)?;

    Ok(Rollback { chain: chain.unwrap_or_default() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("abort" => Ok(Rollback { chain: false }))]
    #[test_case("abort and chain" => Ok(Rollback { chain: true }))]
    #[test_case("abort and no chain" => Ok(Rollback { chain: false }))]
    #[test_case("abort transaction" => Ok(Rollback { chain: false }))]
    #[test_case("abort transaction and chain" => Ok(Rollback { chain: true }))]
    #[test_case("abort transaction and no chain" => Ok(Rollback { chain: false }))]
    fn test_abort(source: &str) -> scan::Result<TransactionStmt> {
        test_parser!(source, abort_stmt)
    }
}

use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Abort;
use pg_parser_core::scan;
use pg_sink_combinators::transaction_chain;
use pg_sink_combinators::work_or_transaction;
use pg_transaction_stmt_ast::TransactionStmt;
use pg_transaction_stmt_ast::TransactionStmt::Rollback;
