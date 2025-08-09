pub(in crate::combinators) fn end_stmt(stream: &mut TokenStream) -> scan::Result<TransactionStmt> {

    /*
    TransactionStmtLegacy:
        END_P ( work_or_transaction )? ( transaction_chain )?
    */

    let (.., chain) = seq!(
        End,
        work_or_transaction.optional(),
        transaction_chain.optional()
            .map(Option::unwrap_or_default)
    ).parse(stream)?;

    Ok(Commit(chain))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::TransactionChain;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("end", TransactionChain::NoChain)]
    #[test_case("end and chain", TransactionChain::WithChain)]
    #[test_case("end and no chain", TransactionChain::NoChain)]
    #[test_case("end transaction", TransactionChain::NoChain)]
    #[test_case("end transaction and chain", TransactionChain::WithChain)]
    #[test_case("end transaction and no chain", TransactionChain::NoChain)]
    fn test_end(source: &str, expected: TransactionChain) {
        test_parser!(source, end_stmt, Commit(expected));
    }
}

use crate::combinators::transaction_chain;
use pg_ast::TransactionStmt;
use pg_ast::TransactionStmt::Commit;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::End;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_combinators::work_or_transaction;
