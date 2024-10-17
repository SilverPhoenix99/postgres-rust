impl Parser<'_> {
    pub(in crate::parser) fn end_stmt(&mut self) -> ParseResult<TransactionStmt> {

        /*
        TransactionStmtLegacy:
            END_P opt_transaction opt_transaction_chain
        */

        self.opt_transaction()?;

        let chain = self.opt_transaction_chain()?;

        Ok(TransactionStmt::Commit { chain })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_end() {
        let mut parser = Parser::new("", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.end_stmt());
    }

    #[test]
    fn test_end_chain() {
        let mut parser = Parser::new("and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: true }), parser.end_stmt());
    }

    #[test]
    fn test_end_no_chain() {
        let mut parser = Parser::new("and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.end_stmt());
    }

    #[test]
    fn test_end_transaction() {
        let mut parser = Parser::new("transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.end_stmt());
    }

    #[test]
    fn test_end_transaction_chain() {
        let mut parser = Parser::new("transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: true }), parser.end_stmt());
    }

    #[test]
    fn test_end_transaction_no_chain() {
        let mut parser = Parser::new("transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.end_stmt());
    }
}

use crate::parser::ast_node::TransactionStmt;
use crate::parser::{ParseResult, Parser};
