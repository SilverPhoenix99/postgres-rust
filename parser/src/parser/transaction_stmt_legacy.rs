/// Alias: `TransactionStmtLegacy`
pub(super) fn transaction_stmt_legacy() -> impl Combinator<Output = TransactionStmt> {
    match_first!{
        begin_stmt(),
        end_stmt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("begin transaction read only", TransactionStmt::Begin(vec![crate::parser::TransactionMode::ReadOnly]))]
    #[test_case("end transaction", TransactionStmt::Commit { chain: false })]
    fn test_transaction(source: &str, expected: TransactionStmt) { {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = transaction_stmt_legacy().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }}
}

use crate::parser::ast_node::TransactionStmt;
use crate::parser::combinators::match_first;
use crate::parser::combinators::Combinator;
use crate::parser::stmt::begin_stmt;
use crate::parser::stmt::end_stmt;
