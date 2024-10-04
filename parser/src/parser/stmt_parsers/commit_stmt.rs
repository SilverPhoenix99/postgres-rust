impl Parser<'_> {
    pub(in crate::parser) fn commit_stmt(&mut self) -> OptResult<TransactionStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Commit))?.is_none() {
            return Ok(None)
        }

        match self.buffer.consume_kw_eq(Unreserved(Prepared)) {
            Err(None) => return Ok(Some(TransactionStmt::Commit { chain: false })),
            Err(Some(err)) => return Err(Some(err)),
            Ok(Some(_)) => {
                let string = self.string().required()?;
                return Ok(Some(TransactionStmt::CommitPrepared(string)))
            }
            Ok(None) => {/* try the next match */}
        }

        self.opt_transaction()?;

        let chain = self.opt_transaction_chain()?;

        Ok(Some(TransactionStmt::Commit { chain }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_commit() {
        let mut parser = Parser::new("commit", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.commit_stmt());
    }

    #[test]
    fn test_commit_chain() {
        let mut parser = Parser::new("commit and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: true })), parser.commit_stmt());
    }

    #[test]
    fn test_commit_no_chain() {
        let mut parser = Parser::new("commit and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.commit_stmt());
    }

    #[test]
    fn test_commit_transaction() {
        let mut parser = Parser::new("commit transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.commit_stmt());
    }

    #[test]
    fn test_commit_transaction_chain() {
        let mut parser = Parser::new("commit transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: true })), parser.commit_stmt());
    }

    #[test]
    fn test_commit_transaction_no_chain() {
        let mut parser = Parser::new("commit transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.commit_stmt());
    }

    #[test]
    fn test_commit_prepared() {
        let mut parser = Parser::new("commit prepared 'test-name'", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::CommitPrepared("test-name".into()))), parser.commit_stmt());
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Commit;
use crate::lexer::UnreservedKeyword::Prepared;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::OptionalResult;
use crate::parser::OptResult;
use crate::parser::Parser;
