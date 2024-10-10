impl Parser<'_> {
    pub(in crate::parser) fn release_savepoint_stmt(&mut self) -> ScanResult<TransactionStmt> {

        /*
        TransactionStmt:
            RELEASE SAVEPOINT ColId
            RELEASE ColId
        */

        self.buffer.consume_kw_eq(Release)?;

        self.buffer.consume_kw_eq(Savepoint).optional()?;

        let name = self.col_id().required()?;

        Ok(TransactionStmt::Release(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_release() {
        let mut parser = Parser::new("release test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Release("test_ident".into())), parser.release_savepoint_stmt());
    }

    #[test]
    fn test_release_savepoint() {
        let mut parser = Parser::new("release savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Release("test_ident".into())), parser.release_savepoint_stmt());
    }
}

use crate::lexer::Keyword::{Release, Savepoint};
use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::{ScanResult, ScanResultTrait};
use crate::parser::Parser;
