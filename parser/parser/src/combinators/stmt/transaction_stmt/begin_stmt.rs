pub(super) fn begin_stmt(ctx: &mut ParserContext) -> scan::Result<TransactionStmt> {

    /*
    TransactionStmtLegacy:
        BEGIN_P ( work_or_transaction )? ( transaction_mode_list )?
    */

    let (.., tx_modes) = seq!(
        Begin,
        work_or_transaction.optional(),
        transaction_mode_list.optional()
    ).parse(ctx)?;

    let tx_modes = tx_modes.unwrap_or_default();
    Ok(TransactionStmt::Begin(tx_modes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::IsolationLevel::Serializable;
    use pg_ast::TransactionMode::Deferrable;
    use pg_ast::TransactionMode::IsolationLevel;
    use pg_ast::TransactionMode::ReadOnly;
    use pg_ast::TransactionMode::ReadWrite;
    use pg_ast::TransactionStmt::Begin;
    use test_case::test_matrix;

    #[test_matrix("begin" => Ok(Begin(Vec::new())))]
    #[test_matrix("begin transaction" => Ok(Begin(Vec::new())))]
    #[test_matrix("begin work" => Ok(Begin(Vec::new())))]
    #[test_matrix("begin read only, read write deferrable" => Ok(Begin(vec![ReadOnly, ReadWrite, Deferrable])))]
    #[test_matrix("begin transaction read write" => Ok(Begin(vec![ReadWrite])))]
    #[test_matrix("begin work isolation level serializable" => Ok(Begin(vec![IsolationLevel(Serializable)])))]
    fn test_begin(source: &str) -> scan::Result<TransactionStmt> {
        test_parser!(source, begin_stmt)
    }
}

use super::transaction_mode_list;
use crate::combinators::core::Combinator;
use crate::combinators::work_or_transaction;
use crate::seq;
use crate::ParserContext;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Begin;
use pg_parser_core::scan;
