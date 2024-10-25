impl Parser<'_> {
    pub(in crate::parser) fn savepoint_stmt(&mut self) -> ParseResult<TransactionStmt> {

        /*
        TransactionStmt:
            SAVEPOINT ColId
        */

        let name = self.col_id().required()?;

        Ok(TransactionStmt::Savepoint(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_savepoint() {
        let mut parser = Parser::new("test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Savepoint("test_ident".into())), parser.savepoint_stmt());
    }
}

use crate::parser::ast_node::TransactionStmt;
use crate::parser::result::Required;
use crate::parser::{ParseResult, Parser};
