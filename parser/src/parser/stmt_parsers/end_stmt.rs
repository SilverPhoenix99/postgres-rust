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

use crate::lexer::Keyword::Reserved;
use crate::lexer::ReservedKeyword::End;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::{OptResult, Parser};

#[cfg(test)]
mod tests {
    use crate::parser::ast_node::TransactionStmt;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::Parser;

    #[test]
    fn test_end() {
        let mut parser = Parser::new(b"end", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.end_stmt());
    }

    #[test]
    fn test_end_chain() {
        let mut parser = Parser::new(b"end and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: true })), parser.end_stmt());
    }

    #[test]
    fn test_end_no_chain() {
        let mut parser = Parser::new(b"end and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.end_stmt());
    }

    #[test]
    fn test_end_transaction() {
        let mut parser = Parser::new(b"end transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.end_stmt());
    }

    #[test]
    fn test_end_transaction_chain() {
        let mut parser = Parser::new(b"end transaction and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: true })), parser.end_stmt());
    }

    #[test]
    fn test_end_transaction_no_chain() {
        let mut parser = Parser::new(b"end transaction and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Commit { chain: false })), parser.end_stmt());
    }
}
