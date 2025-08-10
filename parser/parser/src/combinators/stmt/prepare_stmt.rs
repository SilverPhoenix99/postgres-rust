pub(super) fn prepare_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          PREPARE TRANSACTION SCONST
        | PREPARE ColId ( '(' type_list ')' )? AS PreparableStmt
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
    use pg_combinators::test_parser;

    #[test]
    fn test_prepare_transaction() {
        test_parser!(
            source = "prepare transaction 'some prepared tx'",
            parser = prepare_stmt,
            expected = PrepareTransactionStmt("some prepared tx".into())
        )
    }
}

use pg_ast::RawStmt;
use pg_ast::RawStmt::PrepareTransactionStmt;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Prepare;
use pg_lexer::Keyword::Transaction;
use pg_parser_core::scan;
use pg_sink_combinators::col_id;
