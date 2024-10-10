impl Parser<'_> {
    pub(in crate::parser) fn abort_stmt(&mut self) -> Result<TransactionStmt, ScanErrorKind> {

        /*
        TransactionStmt:
            ABORT_P opt_transaction opt_transaction_chain
        */

        self.buffer.consume_kw_eq(Abort)?;

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
        let mut parser = Parser::new("abort", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.abort_stmt());
    }

    #[test]
    fn test_abort_chain() {
        let mut parser = Parser::new("abort and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: true }), parser.abort_stmt());
    }

    #[test]
    fn test_abort_no_chain() {
        let mut parser = Parser::new("abort and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.abort_stmt());
    }

    #[test]
    fn test_abort_transaction() {
        let mut parser = Parser::new("abort transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.abort_stmt());
    }

    #[test]
    fn test_abort_transaction_chain() {
        let mut parser = Parser::new("abort transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: true }), parser.abort_stmt());
    }

    #[test]
    fn test_abort_transaction_no_chain() {
        let mut parser = Parser::new("abort transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.abort_stmt());
    }
}

use crate::lexer::Keyword::Abort;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::ScanErrorKind;
use crate::parser::Parser;

