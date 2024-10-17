impl Parser<'_> {
    pub(in crate::parser) fn prepare_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            PREPARE TRANSACTION SCONST
            PREPARE ColId ( '(' type_list ')' )? AS PreparableStmt
        */

        let tx = self.buffer.consume_kw_eq(Transaction)
            .no_match_to_option()
            .required()?;

        if tx.is_some() {
            let tx_id = self.string().required()?;
            return Ok(PrepareTransactionStmt(tx_id))
        }

        let name = self.col_id().required()?;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_prepare_transaction() {
        let mut parser = Parser::new("transaction 'some prepared tx'", DEFAULT_CONFIG);
        let expected = PrepareTransactionStmt("some prepared tx".to_string());
        assert_eq!(Ok(expected), parser.prepare_stmt());
    }
}

use crate::lexer::Keyword::Transaction;
use crate::parser::ast_node::RawStmt::{self, PrepareTransactionStmt};
use crate::parser::result::{EofResultTrait, ScanResultTrait};
use crate::parser::{ParseResult, Parser};
