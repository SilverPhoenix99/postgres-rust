impl Parser<'_> {
    pub(in crate::parser) fn rollback_stmt(&mut self) -> OptResult<TransactionStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Rollback))?.is_none() {
            return Ok(None)
        }

        match self.buffer.consume_kw_eq(Unreserved(Prepared)) {
            Err(None) => return Ok(Some(TransactionStmt::Rollback { chain: false })),
            Err(Some(err)) => return Err(Some(err)),
            Ok(Some(_)) => {
                let string = self.string().required()?;
                return Ok(Some(TransactionStmt::RollbackPrepared(string)))
            }
            Ok(None) => {/* try the next match */}
        }

        self.opt_transaction()?;

        match self.buffer.consume_kw_eq(Reserved(To)) {
            Err(None) => return Ok(Some(TransactionStmt::Rollback { chain: false })),
            Err(Some(err)) => return Err(Some(err)),
            Ok(Some(_)) => {
                self.buffer.consume_kw_eq(Unreserved(Savepoint))
                    .map_eof(||
                        Err(Some(ParserErrorKind::default()))
                    )?;

                let name = self.col_id().required()?;
                return Ok(Some(TransactionStmt::RollbackTo(name)))
            },
            Ok(None) => {/* try the next match */},
        };

        let chain = self.opt_transaction_chain()?;
        Ok(Some(TransactionStmt::Rollback { chain }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_rollback() {
        let mut parser = Parser::new("rollback", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: false })), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_chain() {
        let mut parser = Parser::new("rollback and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: true })), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_no_chain() {
        let mut parser = Parser::new("rollback and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: false })), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_to() {
        let mut parser = Parser::new("rollback to test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::RollbackTo("test_ident".into()))), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_to_savepoint() {
        let mut parser = Parser::new("rollback to savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::RollbackTo("test_ident".into()))), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction() {
        let mut parser = Parser::new("rollback transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: false })), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_chain() {
        let mut parser = Parser::new("rollback transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: true })), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_no_chain() {
        let mut parser = Parser::new("rollback transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Rollback { chain: false })), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_to() {
        let mut parser = Parser::new("rollback transaction to test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::RollbackTo("test_ident".into()))), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_transaction_to_savepoint() {
        let mut parser = Parser::new("rollback transaction to savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::RollbackTo("test_ident".into()))), parser.rollback_stmt());
    }

    #[test]
    fn test_rollback_prepared() {
        let mut parser = Parser::new("rollback prepared 'test-string'", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::RollbackPrepared("test-string".into()))), parser.rollback_stmt());
    }
}

use crate::lexer::{
    Keyword::{Reserved, Unreserved},
    ReservedKeyword::To,
    UnreservedKeyword::{Prepared, Rollback, Savepoint},
};
use crate::parser::{
    ast_node::TransactionStmt,
    result::OptionalResult,
    OptResult,
    Parser,
    ParserErrorKind,
};
