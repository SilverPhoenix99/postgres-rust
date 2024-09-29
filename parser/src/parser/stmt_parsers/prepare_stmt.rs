impl Parser<'_> {
    pub(in crate::parser) fn prepare_stmt(&mut self) -> OptResult<AstNode> {

        /*
        prepare_stmt :
              PREPARE TRANSACTION SCONST # TransactionStmt
            | PREPARE ColId prep_type_clause AS PreparableStmt
        */

        if self.buffer.consume_kw_eq(Unreserved(Prepare))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Prepare;
use crate::parser::{AstNode, OptResult, Parser};
