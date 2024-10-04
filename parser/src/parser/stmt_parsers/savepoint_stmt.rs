impl Parser<'_> {
    pub(in crate::parser) fn savepoint_stmt(&mut self) -> OptResult<TransactionStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Savepoint))?.is_none() {
            return Ok(None)
        }

        let name = self.col_id().required()?;

        Ok(Some(TransactionStmt::Savepoint(name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_savepoint() {
        let mut parser = Parser::new("savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Savepoint("test_ident".into()))), parser.savepoint_stmt());
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Savepoint;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::OptionalResult;
use crate::parser::OptResult;
use crate::parser::Parser;
