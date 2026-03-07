/// Alias: `TransactionStmtLegacy`
pub(in crate::combinators) fn transaction_stmt_legacy(ctx: &mut ParserContext) -> scan::Result<TransactionStmt> {

    alt!(
        begin_stmt,
        end_stmt
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::{
        TransactionMode::ReadOnly,
        TransactionStmt::{Begin, Commit}
    };
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("begin transaction read only" => Ok(Begin(vec![ReadOnly])))]
    #[test_case("end transaction" => Ok(Commit { chain: false }))]
    fn test_transaction(source: &str) -> scan::Result<TransactionStmt> {
        test_parser!(source, transaction_stmt_legacy)
    }
}

use super::begin_stmt;
use super::end_stmt;
use pg_ast::TransactionStmt;
use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_parser_core::scan;
