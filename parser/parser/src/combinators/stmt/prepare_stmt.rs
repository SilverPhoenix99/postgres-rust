pub(super) fn prepare_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        PREPARE TRANSACTION SCONST
        PREPARE ColId ( '(' type_list ')' )? AS PreparableStmt
    */

    Prepare
        .and_right(or(
            Transaction
                .and_then(parser(string), |_, tx_id| PrepareTransactionStmt(tx_id)),
            col_id().map(|_name| todo!())
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_prepare_transaction() {
        let mut stream = TokenStream::new("prepare transaction 'some prepared tx'", DEFAULT_CONFIG);
        let expected = PrepareTransactionStmt("some prepared tx".into());
        assert_eq!(Ok(expected), prepare_stmt().parse(&mut stream));
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::or;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::RawStmt;
use pg_ast::RawStmt::PrepareTransactionStmt;
use pg_lexer::Keyword::Prepare;
use pg_lexer::Keyword::Transaction;
