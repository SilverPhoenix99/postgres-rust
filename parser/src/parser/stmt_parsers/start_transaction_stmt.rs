impl Parser<'_> {
    pub(in crate::parser) fn start_transaction_stmt(&mut self) -> OptResult<TransactionStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Start))?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Unreserved(Transaction)).required()?;

        let modes = self.opt_transaction_mode_list().replace_eof(Ok(None))?
            .unwrap_or_else(Vec::new);

        Ok(Some(TransactionStmt::Start(modes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::TransactionMode;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_start_transaction() {
        let mut parser = Parser::new("start transaction", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Start(Vec::new()))), parser.start_transaction_stmt());
    }

    #[test]
    fn test_start_transaction_with_transaction_modes() {
        let mut parser = Parser::new("start transaction read only, read write deferrable", DEFAULT_CONFIG);
        let modes = vec![TransactionMode::ReadOnly, TransactionMode::ReadWrite, TransactionMode::Deferrable];
        assert_eq!(Ok(Some(TransactionStmt::Start(modes))), parser.start_transaction_stmt());
    }
}

use crate::lexer::{
    Keyword::Unreserved,
    UnreservedKeyword::{Start, Transaction},
};
use crate::parser::{
    ast_node::TransactionStmt,
    result::OptionalResult,
    OptResult,
    Parser,
};
