impl Parser<'_> {
    pub(in crate::parser) fn begin_stmt(&mut self) -> OptResult<TransactionStmt> {

        /*
        TransactionStmtLegacy:
            BEGIN_P opt_transaction opt_transaction_mode_list
        */

        if self.buffer.consume_kw_eq(Begin)?.is_none() {
            return Ok(None)
        }

        self.opt_transaction()?;

        let modes = self.opt_transaction_mode_list()
            .replace_eof(Ok(None))?
            .unwrap_or_else(Vec::new);

        Ok(Some(TransactionStmt::Start(modes)))
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
        assert_eq!(Ok(Some(TransactionStmt::Start(Vec::new()))), parser.begin_stmt());
    }

    #[test]
    fn test_begin_transaction() {
        let mut parser = Parser::new("begin transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Start(Vec::new()))), parser.begin_stmt());
    }

    #[test]
    fn test_begin_work() {
        let mut parser = Parser::new("begin work", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Start(Vec::new()))), parser.begin_stmt());
    }

    #[test]
    fn test_begin_with_transaction_modes() {
        let mut parser = Parser::new("begin read only, read write deferrable", DEFAULT_CONFIG);
        let modes = vec![ReadOnly, ReadWrite, Deferrable];
        assert_eq!(Ok(Some(TransactionStmt::Start(modes))), parser.begin_stmt());
    }

    #[test]
    fn test_begin_transaction_with_transaction_modes() {
        let mut parser = Parser::new("begin transaction read write", DEFAULT_CONFIG);
        let modes = vec![ReadWrite];
        assert_eq!(Ok(Some(TransactionStmt::Start(modes))), parser.begin_stmt());
    }

    #[test]
    fn test_begin_work_with_transaction_modes() {
        let mut parser = Parser::new("begin work isolation level serializable", DEFAULT_CONFIG);
        let modes = vec![IsolationLevel(Serializable)];
        assert_eq!(Ok(Some(TransactionStmt::Start(modes))), parser.begin_stmt());
    }
}

use crate::lexer::Keyword::Begin;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::OptionalResult;
use crate::parser::OptResult;
use crate::parser::Parser;
