impl Parser<'_> {
    pub(in crate::parser) fn begin_stmt(&mut self) -> OptResult<TransactionStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Begin))?.is_none() {
            return Ok(None)
        }

        self.opt_transaction()?;

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Begin;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::{OptResult, Parser};
