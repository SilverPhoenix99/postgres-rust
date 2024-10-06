impl Parser<'_> {
    pub(in crate::parser) fn release_savepoint_stmt(&mut self) -> OptResult<TransactionStmt> {

        /*
        TransactionStmt:
            RELEASE SAVEPOINT ColId
            RELEASE ColId
        */

        if self.buffer.consume_kw_eq(Release)?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Savepoint).replace_eof(Ok(None))?;

        let name = self.col_id().required()?;

        Ok(Some(TransactionStmt::Release(name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_release() {
        let mut parser = Parser::new("release test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Release("test_ident".into()))), parser.release_savepoint_stmt());
    }

    #[test]
    fn test_release_savepoint() {
        let mut parser = Parser::new("release savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(TransactionStmt::Release("test_ident".into()))), parser.release_savepoint_stmt());
    }
}

use crate::lexer::Keyword::{Release, Savepoint};
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::OptionalResult;
use crate::parser::OptResult;
use crate::parser::Parser;
