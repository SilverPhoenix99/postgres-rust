pub(in crate::combinators) fn end_stmt(ctx: &mut ParserContext) -> scan::Result<TransactionStmt> {

    /*
    TransactionStmtLegacy:
        END_P ( work_or_transaction )? ( transaction_chain )?
    */

    let (.., chain) = seq!(
        End,
        work_or_transaction.optional(),
        transaction_chain.optional()
    ).parse(ctx)?;

    Ok(Commit { chain: chain.unwrap_or_default() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("end" => Ok(Commit { chain: false }))]
    #[test_case("end and chain" => Ok(Commit { chain: true }))]
    #[test_case("end and no chain" => Ok(Commit { chain: false }))]
    #[test_case("end transaction" => Ok(Commit { chain: false }))]
    #[test_case("end transaction and chain" => Ok(Commit { chain: true }))]
    #[test_case("end transaction and no chain" => Ok(Commit { chain: false }))]
    fn test_end(source: &str) -> scan::Result<TransactionStmt> {
        test_parser!(source, end_stmt)
    }
}

use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::End;
use pg_parser_core::scan;
use pg_sink_combinators::transaction_chain;
use pg_sink_combinators::work_or_transaction;
use pg_transaction_stmt_ast::TransactionStmt;
use pg_transaction_stmt_ast::TransactionStmt::Commit;
