pub(in crate::combinators) fn end_stmt(stream: &mut TokenStream) -> Result<TransactionStmt> {

    /*
    TransactionStmtLegacy:
        END_P opt_transaction opt_transaction_chain
    */

    seq!(stream =>
        End,
        opt_transaction,
        opt_transaction_chain
    )
        .map(|(.., chain)| Commit { chain })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("end", false)]
    #[test_case("end and chain", true)]
    #[test_case("end and no chain", false)]
    #[test_case("end transaction", false)]
    #[test_case("end transaction and chain", true)]
    #[test_case("end transaction and no chain", false)]
    fn test_end(source: &str, expected: bool) {
        test_parser!(source, end_stmt, Commit { chain: expected });
    }
}

use crate::combinators::foundation::{seq, Combinator};
use crate::combinators::opt_transaction;
use crate::combinators::opt_transaction_chain;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::TransactionStmt;
use pg_ast::TransactionStmt::Commit;
use pg_lexer::Keyword::End;
