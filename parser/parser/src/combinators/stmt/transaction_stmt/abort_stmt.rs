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
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("abort" => Ok(Rollback { chain: false }))]
    #[test_matrix("abort and chain" => Ok(Rollback { chain: true }))]
    #[test_matrix("abort and no chain" => Ok(Rollback { chain: false }))]
    #[test_matrix("abort transaction" => Ok(Rollback { chain: false }))]
    #[test_matrix("abort transaction and chain" => Ok(Rollback { chain: true }))]
    #[test_matrix("abort transaction and no chain" => Ok(Rollback { chain: false }))]
    fn test_abort(source: &str) -> scan::Result<TransactionStmt> {
        test_parser!(source, abort_stmt)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::transaction_chain;
use crate::combinators::work_or_transaction;
use crate::seq;
use crate::ParserContext;
use pg_ast::TransactionStmt;
use pg_ast::TransactionStmt::Rollback;
use pg_lexer::Keyword::Abort;
use pg_parser_core::scan;
