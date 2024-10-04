impl Parser<'_> {
    pub(in crate::parser) fn end_stmt(&mut self) -> OptResult<TransactionStmt> {

        if self.buffer.consume_kw_eq(Reserved(End))?.is_none() {
            return Ok(None)
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
    fn test_end() {
        let mut parser = Parser::new("end", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.end_stmt());
    }

    #[test]
    fn test_end_chain() {
        let mut parser = Parser::new("end and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: true })), parser.end_stmt());
    }

    #[test]
    fn test_end_no_chain() {
        let mut parser = Parser::new("end and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.end_stmt());
    }

    #[test]
    fn test_end_transaction() {
        let mut parser = Parser::new("end transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.end_stmt());
    }

    #[test]
    fn test_end_transaction_chain() {
        let mut parser = Parser::new("end transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: true })), parser.end_stmt());
    }

    #[test]
    fn test_end_transaction_no_chain() {
        let mut parser = Parser::new("end transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.end_stmt());
    }
}

use crate::lexer::Keyword::Reserved;
use crate::lexer::ReservedKeyword::End;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::OptResult;
use crate::parser::Parser;
