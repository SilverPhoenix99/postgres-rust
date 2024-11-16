impl Parser<'_> {
    pub(in crate::parser) fn rollback_stmt(&mut self) -> ParseResult<TransactionStmt> {

        /*
            ROLLBACK PREPARED SCONST
            ROLLBACK opt_transaction opt_transaction_chain
            ROLLBACK opt_transaction TO SAVEPOINT ColId
            ROLLBACK opt_transaction TO ColId
        */

        if self.buffer.consume_kw_eq(Prepared).optional()?.is_some() {
            /*
                ROLLBACK PREPARED SCONST
            */
            let string = self.string().required(fn_info!())?;
            return Ok(TransactionStmt::RollbackPrepared(string))
        }

        self.opt_transaction()?;

        match self.buffer.consume_kw_eq(To) {
            Ok(_) => {
                /*
                    ROLLBACK opt_transaction TO SAVEPOINT ColId
                    ROLLBACK opt_transaction TO ColId
                */
                self.buffer.consume_kw_eq(Savepoint).optional()?;
                let name = self.col_id().required(fn_info!())?;
                Ok(TransactionStmt::RollbackTo(name))
            },
            Err(Eof(_)) => {
                /*
                    ROLLBACK opt_transaction
                */
                Ok(TransactionStmt::Rollback { chain: false })
            },
            Err(ScanErr(err)) => Err(err),
            Err(NoMatch(_)) => {
                /*
                    ROLLBACK opt_transaction opt_transaction_chain
                */
                let chain = self.opt_transaction_chain()?;
                Ok(TransactionStmt::Rollback { chain })
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_rollback() {
        let mut parser = Parser::new("", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_chain() {
        let mut parser = Parser::new("and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: true }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_no_chain() {
        let mut parser = Parser::new("and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_to() {
        let mut parser = Parser::new("to test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::RollbackTo("test_ident".into())), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_to_savepoint() {
        let mut parser = Parser::new("to savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::RollbackTo("test_ident".into())), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction() {
        let mut parser = Parser::new("transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_chain() {
        let mut parser = Parser::new("transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: true }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_no_chain() {
        let mut parser = Parser::new("transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_to() {
        let mut parser = Parser::new("transaction to test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::RollbackTo("test_ident".into())), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_to_savepoint() {
        let mut parser = Parser::new("transaction to savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::RollbackTo("test_ident".into())), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_prepared() {
        let mut parser = Parser::new("prepared 'test-string'", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::RollbackPrepared("test-string".into())), parser.rollback_stmt());
    }
}

use crate::lexer::Keyword::{Prepared, Savepoint, To};
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::ScanErrorKind::{Eof, NoMatch, ScanErr};
use crate::parser::result::{Optional, Required};
use crate::parser::{ParseResult, Parser};
use postgres_basics::fn_info;
