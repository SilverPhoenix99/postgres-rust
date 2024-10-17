impl Parser<'_> {
    pub(in crate::parser) fn abort_stmt(&mut self) -> ParseResult<TransactionStmt> {

        /*
        TransactionStmt:
            ABORT_P opt_transaction opt_transaction_chain
        */

        self.opt_transaction()?;

        let chain = self.opt_transaction_chain()?;

        Ok(TransactionStmt::Rollback { chain })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::Parser;

    #[test]
    fn test_abort() {
        let mut parser = Parser::new("", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.abort_stmt());
    }

    #[test]
    fn test_abort_chain() {
        let mut parser = Parser::new("and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: true }), parser.abort_stmt());
    }

    #[test]
    fn test_abort_no_chain() {
        let mut parser = Parser::new("and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.abort_stmt());
    }

    #[test]
    fn test_abort_transaction() {
        let mut parser = Parser::new("transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.abort_stmt());
    }

    #[test]
    fn test_abort_transaction_chain() {
        let mut parser = Parser::new("transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: true }), parser.abort_stmt());
    }

    #[test]
    fn test_abort_transaction_no_chain() {
        let mut parser = Parser::new("transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.abort_stmt());
    }
}

use crate::parser::ast_node::TransactionStmt;
use crate::parser::{ParseResult, Parser};
