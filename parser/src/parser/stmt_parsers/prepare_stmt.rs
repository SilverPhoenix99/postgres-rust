impl Parser<'_> {
    pub(in crate::parser) fn prepare_stmt(&mut self) -> OptResult<AstNode> {

        // TODO 
        /*
            PREPARE TRANSACTION SCONST
            PREPARE ColId prep_type_clause AS PreparableStmt
        */

        if self.buffer.consume_kw_eq(Prepare)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Prepare;
use crate::parser::{AstNode, OptResult, Parser};
