pub(in crate::combinators) fn begin_stmt(stream: &mut TokenStream) -> scan::Result<TransactionStmt> {

    /*
    TransactionStmtLegacy:
        BEGIN_P ( work_or_transaction )? ( transaction_mode_list )?
    */

    let (.., tx_modes) = seq!(
        Begin,
        work_or_transaction.optional(),
        transaction_mode_list.optional()
    ).parse(stream)?;

    let tx_modes = tx_modes.unwrap_or_default();
    Ok(TransactionStmt::Begin(tx_modes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
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

use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::transaction_mode_list;
use crate::combinators::work_or_transaction;
use crate::stream::TokenStream;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Begin;
use pg_parser_core::scan;
