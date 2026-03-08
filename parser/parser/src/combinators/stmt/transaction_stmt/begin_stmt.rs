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
    use pg_ast::IsolationLevel::*;
    use pg_ast::TransactionMode::{self, *};
    use test_case::test_case;
    use TransactionStmt::Begin;

    #[test_case("begin", Vec::new())]
    #[test_case("begin transaction", Vec::new())]
    #[test_case("begin work", Vec::new())]
    #[test_case("begin read only, read write deferrable", vec![ReadOnly, ReadWrite, Deferrable])]
    #[test_case("begin transaction read write", vec![ReadWrite])]
    #[test_case("begin work isolation level serializable", vec![IsolationLevel(Serializable)])]
    fn test_begin(source: &str, expected: Vec<TransactionMode>) {
        test_parser!(source, begin_stmt, Begin(expected))
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
