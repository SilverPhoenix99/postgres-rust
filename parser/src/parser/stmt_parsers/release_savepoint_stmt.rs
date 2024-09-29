impl Parser<'_> {
    pub(in crate::parser) fn release_savepoint_stmt(&mut self) -> OptResult<TransactionStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Release))?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Unreserved(Savepoint)).replace_eof(Ok(None))?;

        let name = self.col_id().required()?;

        Ok(Some(TransactionStmt::Release(name)))
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::{Release, Savepoint};
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::OptionalResult;
use crate::parser::{OptResult, Parser};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_release() {
        let mut parser = Parser::new(b"release test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Release("test_ident".into()))), parser.release_savepoint_stmt());
    }

    #[test]
    fn test_release_savepoint() {
        let mut parser = Parser::new(b"release savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Release("test_ident".into()))), parser.release_savepoint_stmt());
    }
}
