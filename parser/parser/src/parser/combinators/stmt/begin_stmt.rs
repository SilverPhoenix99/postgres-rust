pub(in crate::parser::combinators) fn begin_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
    TransactionStmtLegacy:
        BEGIN_P opt_transaction opt_transaction_mode_list
    */

    sequence!(
        Begin.and(opt_transaction()).skip(),
        transaction_mode_list().optional()
    ).map(|(_, tx_modes)|
        TransactionStmt::Begin(tx_modes.unwrap_or_default())
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::IsolationLevel::*;
    use crate::parser::ast_node::TransactionMode::{self, *};
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;
    use TransactionStmt::Begin;

    #[test_case("begin", Vec::new())]
    #[test_case("begin transaction", Vec::new())]
    #[test_case("begin work", Vec::new())]
    #[test_case("begin read only, read write deferrable", vec![ReadOnly, ReadWrite, Deferrable])]
    #[test_case("begin transaction read write", vec![ReadWrite])]
    #[test_case("begin work isolation level serializable", vec![IsolationLevel(Serializable)])]
    fn test_begin(source: &str, expected: Vec<TransactionMode>) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(Begin(expected)), begin_stmt().parse(&mut stream));
    }
}

use crate::parser::ast_node::TransactionStmt;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::opt_transaction;
use crate::parser::combinators::transaction_mode_list;
use postgres_parser_lexer::Keyword::Begin;
