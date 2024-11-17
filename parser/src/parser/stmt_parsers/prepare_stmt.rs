impl Parser<'_> {
    pub(in crate::parser) fn prepare_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            PREPARE TRANSACTION SCONST
            PREPARE ColId ( '(' type_list ')' )? AS PreparableStmt
        */

        let tx = self.buffer.consume_kw_eq(Transaction).try_match(fn_info!())?;

        if tx.is_some() {
            let tx_id = string(fn_info!())
                .required(fn_info!())
                .parse(&mut self.buffer)?;
            return Ok(PrepareTransactionStmt(tx_id))
        }

        let name = self.col_id().required(fn_info!())?;
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
        combinators::{string, ParserFunc, ParserFuncHelpers},
        result::{Required, TryMatch},
        ParseResult,
        Parser
    },
};
use postgres_basics::fn_info;
