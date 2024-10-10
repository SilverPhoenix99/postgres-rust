impl Parser<'_> {
    pub(in crate::parser) fn prepare_stmt(&mut self) -> Result<AstNode, ScanErrorKind> {

        /*
            PREPARE TRANSACTION SCONST
            PREPARE ColId ( '(' type_list ')' )? AS PreparableStmt
        */

        self.buffer.consume_kw_eq(Prepare)?;

        let tx = self.buffer.consume_kw_eq(Transaction)
            .no_match_to_option()
            .required()?;

        if tx.is_some() {
            let tx_id = self.string().required()?;
            return Ok(PrepareTransaction(tx_id))
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
        let mut parser = Parser::new("prepare transaction 'some prepared tx'", DEFAULT_CONFIG);
        let expected = PrepareTransaction("some prepared tx".to_string());
        assert_eq!(Ok(expected), parser.prepare_stmt());
    }
}

use crate::lexer::Keyword::{Prepare, Transaction};
use crate::parser::ast_node::AstNode::{self, PrepareTransaction};
use crate::parser::result::{EofResult, ScanErrorKind, ScanResult};
use crate::parser::Parser;
