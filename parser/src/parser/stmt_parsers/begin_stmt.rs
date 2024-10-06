impl Parser<'_> {
    pub(in crate::parser) fn begin_stmt(&mut self) -> OptResult<TransactionStmt> {

        /*
        TransactionStmtLegacy:
            BEGIN_P opt_transaction opt_transaction_mode_list
        */

        if self.buffer.consume_kw_eq(Begin)?.is_none() {
            return Ok(None)
        }

        self.opt_transaction()?;

        todo!()
    }
}

use crate::lexer::Keyword::Begin;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::OptResult;
use crate::parser::Parser;
