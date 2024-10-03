impl Parser<'_> {
    pub(in crate::parser) fn abort_stmt(&mut self) -> OptResult<TransactionStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Abort))?.is_none() {
            return Ok(None)
        }

        self.opt_transaction()?;

        let chain = self.opt_transaction_chain()?;

        Ok(Some(TransactionStmt::Rollback { chain }))
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Abort;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::{OptResult, Parser};

#[cfg(test)]
mod tests {
    use crate::parser::ast_node::TransactionStmt;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::Parser;

    #[test]
    fn test_abort() {
        let mut parser = Parser::new("abort", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: false })), parser.abort_stmt());
    }

    #[test]
    fn test_abort_chain() {
        let mut parser = Parser::new("abort and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: true })), parser.abort_stmt());
    }

    #[test]
    fn test_abort_no_chain() {
        let mut parser = Parser::new("abort and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: false })), parser.abort_stmt());
    }

    #[test]
    fn test_abort_transaction() {
        let mut parser = Parser::new("abort transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: false })), parser.abort_stmt());
    }

    #[test]
    fn test_abort_transaction_chain() {
        let mut parser = Parser::new("abort transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: true })), parser.abort_stmt());
    }

    #[test]
    fn test_abort_transaction_no_chain() {
        let mut parser = Parser::new("abort transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: false })), parser.abort_stmt());
    }
}
