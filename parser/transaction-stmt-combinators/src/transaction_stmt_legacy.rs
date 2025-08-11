/// Alias: `TransactionStmtLegacy`
pub fn transaction_stmt_legacy(ctx: &mut ParserContext) -> scan::Result<TransactionStmt> {

    alt!(
        begin_stmt,
        end_stmt
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    #[allow(unused_imports)]
    use pg_transaction_stmt_ast::{
        TransactionMode::ReadOnly,
        TransactionStmt::{Begin, Commit}
    };
    use test_case::test_case;

    #[test_case("begin transaction read only" => Ok(Begin(vec![ReadOnly])))]
    #[test_case("end transaction" => Ok(Commit { chain: false }))]
    fn test_transaction(source: &str) -> scan::Result<TransactionStmt> {
        test_parser!(source, transaction_stmt_legacy)
    }
}

use crate::begin_stmt;
use crate::end_stmt;
use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_parser_core::scan;
use pg_transaction_stmt_ast::TransactionStmt;
