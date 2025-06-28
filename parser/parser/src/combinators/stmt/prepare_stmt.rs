pub(super) fn prepare_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        PREPARE TRANSACTION SCONST
        PREPARE ColId ( '(' type_list ')' )? AS PreparableStmt
    */

    let (_, stmt) = seq!(=>
        Prepare.parse(stream),
        choice!(stream =>
            seq!(stream => Transaction, string)
                .map(|(_, tx_id)| PrepareTransactionStmt(tx_id)),
            col_id(stream)
                .map(|_name| todo!())
        )
    )?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_prepare_transaction() {
        test_parser!(
            source = "prepare transaction 'some prepared tx'",
            parser = prepare_stmt,
            expected = PrepareTransactionStmt("some prepared tx".into())
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_ast::RawStmt::PrepareTransactionStmt;
use pg_lexer::Keyword::Prepare;
use pg_lexer::Keyword::Transaction;
