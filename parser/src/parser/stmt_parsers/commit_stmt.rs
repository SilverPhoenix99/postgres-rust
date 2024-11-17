impl Parser<'_> {
    pub(in crate::parser) fn commit_stmt(&mut self) -> ParseResult<TransactionStmt> {

        /*
            COMMIT opt_transaction opt_transaction_chain
            COMMIT PREPARED SCONST
        */

        if keyword(Prepared).optional().parse(&mut self.buffer)?.is_some() {
            let string = string(fn_info!()).required(fn_info!()).parse(&mut self.buffer)?;
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
        let mut parser = Parser::new("", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_chain() {
        let mut parser = Parser::new("and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: true }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_no_chain() {
        let mut parser = Parser::new("and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_transaction() {
        let mut parser = Parser::new("transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_transaction_chain() {
        let mut parser = Parser::new("transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: true }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_transaction_no_chain() {
        let mut parser = Parser::new("transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Commit { chain: false }), parser.commit_stmt());
    }

    #[test]
    fn test_commit_prepared() {
        let mut parser = Parser::new("prepared 'test-name'", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::CommitPrepared("test-name".into())), parser.commit_stmt());
    }
}

use crate::lexer::Keyword::Prepared;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::combinators::{keyword, string, ParserFunc, ParserFuncHelpers};
use crate::parser::{ParseResult, Parser};
use postgres_basics::fn_info;
