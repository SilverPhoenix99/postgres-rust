impl Parser<'_> {
    pub(in crate::parser) fn prepare_stmt(&mut self) -> ParseResult<RawStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::prepare_stmt";

        /*
            PREPARE TRANSACTION SCONST
            PREPARE ColId ( '(' type_list ')' )? AS PreparableStmt
        */

        let tx = self.buffer.consume_kw_eq(Transaction).try_match(fn_info!(FN_NAME))?;

        if tx.is_some() {
            let tx_id = self.string().required(fn_info!(FN_NAME))?;
            return Ok(PrepareTransactionStmt(tx_id))
        }

        let name = self.col_id().required(fn_info!(FN_NAME))?;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_prepare_transaction() {
        let mut parser = Parser::new("transaction 'some prepared tx'", DEFAULT_CONFIG);
        let expected = PrepareTransactionStmt("some prepared tx".into());
        assert_eq!(Ok(expected), parser.prepare_stmt());
    }
}

use crate::{
    lexer::Keyword::Transaction,
    parser::{
        ast_node::RawStmt::{self, PrepareTransactionStmt},
        result::{Required, TryMatch},
        ParseResult,
        Parser
    },
};
use postgres_basics::fn_info;
