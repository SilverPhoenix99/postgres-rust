impl Parser<'_> {
    pub(in crate::parser) fn start_transaction_stmt(&mut self) -> Result<TransactionStmt, ScanErrorKind> {

        /*
        TransactionStmt:
            START TRANSACTION transaction_mode_list_or_empty
        */

        self.buffer.consume_kw_eq(Start)?;
        self.buffer.consume_kw_eq(Transaction).required()?;

        let modes = self.transaction_mode_list()
            .optional()?
            .unwrap_or_else(Vec::new);

        Ok(TransactionStmt::Start(modes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::TransactionMode::{Deferrable, ReadOnly, ReadWrite};
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_start_transaction() {
        let mut parser = Parser::new("start transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Start(Vec::new())), parser.start_transaction_stmt());
    }

    #[test]
    fn test_start_transaction_with_transaction_modes() {
        let mut parser = Parser::new("start transaction read only, read write deferrable", DEFAULT_CONFIG);
        let modes = vec![ReadOnly, ReadWrite, Deferrable];
        assert_eq!(Ok(TransactionStmt::Start(modes)), parser.start_transaction_stmt());
    }
}

use crate::lexer::Keyword::{Start, Transaction};
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::{ScanErrorKind, ScanResult};
use crate::parser::Parser;
