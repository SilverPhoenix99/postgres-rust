pub(super) fn prepare_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        PREPARE TRANSACTION SCONST
        PREPARE ColId ( '(' type_list ')' )? AS PreparableStmt
    */

    Prepare
        .and_right(or(
            Transaction
                .and_then(string(), |_, tx_id| PrepareTransactionStmt(tx_id)),
            col_id().map(|_name| todo!())
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_prepare_transaction() {
        let mut stream = TokenStream::new("prepare transaction 'some prepared tx'", DEFAULT_CONFIG);
        let expected = PrepareTransactionStmt("some prepared tx".into());
        assert_eq!(Ok(expected), prepare_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::{Prepare, Transaction};
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RawStmt::PrepareTransactionStmt;
use crate::parser::col_id;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::combinators::{or, string};
