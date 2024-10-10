impl Parser<'_> {
    pub(in crate::parser) fn rollback_stmt(&mut self) -> ScanResult<TransactionStmt> {

        /*
            ROLLBACK PREPARED SCONST
            ROLLBACK opt_transaction opt_transaction_chain
            ROLLBACK opt_transaction TO SAVEPOINT ColId
            ROLLBACK opt_transaction TO ColId
        */

        self.buffer.consume_kw_eq(Rollback)?;

        if self.buffer.consume_kw_eq(Prepared).optional()?.is_some() {
            /*
                ROLLBACK PREPARED SCONST
            */
            let string = self.string().required()?;
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
                let name = self.col_id().required()?;
                Ok(TransactionStmt::RollbackTo(name))
            },
            Err(Eof) => {
                /*
                    ROLLBACK opt_transaction
                */
                Ok(TransactionStmt::Rollback { chain: false })
            },
            Err(ParserErr(err)) => Err(ParserErr(err)),
            Err(NoMatch) => {
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
    use crate::parser::Parser;

    #[test]
    fn test_rollback() {
        let mut parser = Parser::new("rollback", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_chain() {
        let mut parser = Parser::new("rollback and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: true }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_no_chain() {
        let mut parser = Parser::new("rollback and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_to() {
        let mut parser = Parser::new("rollback to test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::RollbackTo("test_ident".into())), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_to_savepoint() {
        let mut parser = Parser::new("rollback to savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::RollbackTo("test_ident".into())), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction() {
        let mut parser = Parser::new("rollback transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_chain() {
        let mut parser = Parser::new("rollback transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: true }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_no_chain() {
        let mut parser = Parser::new("rollback transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: false }), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_to() {
        let mut parser = Parser::new("rollback transaction to test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::RollbackTo("test_ident".into())), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_to_savepoint() {
        let mut parser = Parser::new("rollback transaction to savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::RollbackTo("test_ident".into())), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_prepared() {
        let mut parser = Parser::new("rollback prepared 'test-string'", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::RollbackPrepared("test-string".into())), parser.rollback_stmt());
    }
}

use crate::lexer::Keyword::{Prepared, Rollback, Savepoint, To};
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::ScanErrorKind::{Eof, NoMatch, ParserErr};
use crate::parser::result::{ScanResult, ScanResultTrait};
use crate::parser::Parser;
