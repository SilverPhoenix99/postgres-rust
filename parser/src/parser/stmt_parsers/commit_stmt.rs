impl Parser<'_> {
    pub(in crate::parser) fn commit_stmt(&mut self) -> ScanResult<TransactionStmt> {

        /*
            COMMIT opt_transaction opt_transaction_chain
            COMMIT PREPARED SCONST
        */

        self.buffer.consume_kw_eq(Commit)?;

        if self.buffer.consume_kw_eq(Prepared).optional()?.is_some() {
            let string = self.string().required()?;
            return Ok(TransactionStmt::CommitPrepared(string))
        }

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
    fn test_commit() {
        let mut parser = Parser::new("commit", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_chain() {
        let mut parser = Parser::new("commit and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: true }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_no_chain() {
        let mut parser = Parser::new("commit and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_transaction() {
        let mut parser = Parser::new("commit transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_transaction_chain() {
        let mut parser = Parser::new("commit transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: true }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_transaction_no_chain() {
        let mut parser = Parser::new("commit transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_prepared() {
        let mut parser = Parser::new("commit prepared 'test-name'", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::CommitPrepared("test-name".into())), parser.commit_stmt());
    }
}

use crate::lexer::Keyword::{Commit, Prepared};
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::{ScanResult, ScanResultTrait};
use crate::parser::Parser;
