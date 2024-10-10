impl Parser<'_> {
    pub(in crate::parser) fn begin_stmt(&mut self) -> ScanResult<TransactionStmt> {

        /*
        TransactionStmtLegacy:
            BEGIN_P opt_transaction opt_transaction_mode_list
        */

        self.buffer.consume_kw_eq(Begin)?;

        self.opt_transaction()?;

        let modes = self.transaction_mode_list()
            .optional()?
            .unwrap_or_else(Vec::new);

        Ok(TransactionStmt::Start(modes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::IsolationLevel::Serializable;
    use crate::parser::ast_node::TransactionMode::{Deferrable, IsolationLevel, ReadOnly, ReadWrite};
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_begin() {
        let mut parser = Parser::new("begin", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Start(Vec::new())), parser.begin_stmt());
    }

    #[test]
    fn test_begin_transaction() {
        let mut parser = Parser::new("begin transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Start(Vec::new())), parser.begin_stmt());
    }

    #[test]
    fn test_begin_work() {
        let mut parser = Parser::new("begin work", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Start(Vec::new())), parser.begin_stmt());
    }

    #[test]
    fn test_begin_with_transaction_modes() {
        let mut parser = Parser::new("begin read only, read write deferrable", DEFAULT_CONFIG);
        let expected_modes = vec![ReadOnly, ReadWrite, Deferrable];
        assert_eq!(Ok(TransactionStmt::Start(expected_modes)), parser.begin_stmt());
    }

    #[test]
    fn test_begin_transaction_with_transaction_modes() {
        let mut parser = Parser::new("begin transaction read write", DEFAULT_CONFIG);
        let expected_modes = vec![ReadWrite];
        assert_eq!(Ok(TransactionStmt::Start(expected_modes)), parser.begin_stmt());
    }

    #[test]
    fn test_begin_work_with_transaction_modes() {
        let mut parser = Parser::new("begin work isolation level serializable", DEFAULT_CONFIG);
        let expected_modes = vec![IsolationLevel(Serializable)];
        assert_eq!(Ok(TransactionStmt::Start(expected_modes)), parser.begin_stmt());
    }
}

use crate::lexer::Keyword::Begin;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::{ScanResult, ScanResultTrait};
use crate::parser::Parser;
