/// Alias: `TransactionStmt`
pub(in crate::combinators::stmt) fn transaction_stmt(ctx: &mut ParserContext) -> scan::Result<TransactionStmt> {

    alt!(
        abort_stmt,
        commit_stmt,
        release_savepoint_stmt,
        rollback_stmt,
        savepoint_stmt,
        start_transaction_stmt,
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_matrix;

    // This only quickly tests that statement types aren't missing.
    // More in-depth testing is within each statement's module.
    #[test_matrix(
        [
            "abort transaction",
            "commit and no chain",
            "release savepoint test_ident",
            "rollback to test_ident",
            "savepoint test_ident",
            "start transaction read only, read write deferrable",
        ]
        => matches Ok(_)
    )]
    fn test_transaction_stmt(source: &str) -> scan::Result<TransactionStmt> {
        test_parser!(source, transaction_stmt)
    }
}

use super::abort_stmt;
use super::commit_stmt;
use super::release_savepoint_stmt;
use super::rollback_stmt;
use super::savepoint_stmt;
use super::start_transaction_stmt;
use pg_ast::TransactionStmt;
use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_parser_core::scan;
