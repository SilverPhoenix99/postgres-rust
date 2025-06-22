pub(in crate::combinators) fn begin_stmt(stream: &mut TokenStream) -> Result<TransactionStmt> {

    /*
    TransactionStmtLegacy:
        BEGIN_P opt_transaction opt_transaction_mode_list
    */

    seq!(
        Begin.and(parser(opt_transaction)).skip(),
        transaction_mode_list.optional()
    ).map(|(_, tx_modes)|
        TransactionStmt::Begin(tx_modes.unwrap_or_default())
    )
        .parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::foundation::ClosureHelpers;
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

use crate::combinators::foundation::{parser, seq};
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::opt_transaction;
use crate::combinators::transaction_mode_list;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Begin;
