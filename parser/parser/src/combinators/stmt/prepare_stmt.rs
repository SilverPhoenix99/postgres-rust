pub(super) fn prepare_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          PREPARE TRANSACTION SCONST                             => TransactionStmt
        | PREPARE ColId ( '(' type_list ')' )? AS PreparableStmt => PrepareStmt
    */

    let (_, stmt) = seq!(
        Prepare,
        alt!(
            seq!(Transaction, string)
                .map(|(_, tx_id)| PrepareTransactionStmt(tx_id)),
            col_id
                .map(|_name| todo!())
        )
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_prepare_transaction() {
        test_parser!(
            source = "prepare transaction 'some prepared tx'",
            parser = prepare_stmt,
            expected = PrepareTransactionStmt("some prepared tx".into())
        )
    }
}

use crate::alt;
use crate::combinators::col_id;
use crate::combinators::core::string;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::RawStmt;
use pg_ast::RawStmt::PrepareTransactionStmt;
use pg_lexer::Keyword::Prepare;
use pg_lexer::Keyword::Transaction;
use pg_parser_core::scan;
