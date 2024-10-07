impl Parser<'_> {
    pub(in crate::parser) fn prepare_stmt(&mut self) -> OptResult<AstNode> {

        /*
            PREPARE TRANSACTION SCONST
            PREPARE ColId ( '(' type_list ')' )? AS PreparableStmt
        */

        if self.buffer.consume_kw_eq(Prepare)?.is_none() {
            return Ok(None)
        }

        let tx = self.buffer.consume_kw_eq(Transaction)
            .replace_eof(
                Err(Some(ParserErrorKind::default()))
            )?;

        if tx.is_some() {
            let tx_id = self.string().required()?;
            return Ok(Some(
                PrepareTransaction(tx_id)
            ))
        }

        let name = self.col_id().required()?;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast_node::AstNode::PrepareTransaction;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::Parser;

    #[test]
    fn test_prepare_transaction() {
        let mut parser = Parser::new("prepare transaction 'some prepared tx'", DEFAULT_CONFIG);
        let expected = PrepareTransaction("some prepared tx".to_string());
        assert_eq!(Ok(Some(expected)), parser.prepare_stmt());
    }
}

use crate::lexer::Keyword::{Prepare, Transaction};
use crate::parser::ast_node::AstNode::PrepareTransaction;
use crate::parser::result::OptionalResult;
use crate::parser::{AstNode, OptResult, Parser, ParserErrorKind};
