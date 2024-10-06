impl Parser<'_> {
    /// Alias: `ImportForeignSchemaStmt`
    pub(in crate::parser) fn import_stmt(&mut self) -> OptResult<AstNode> {

        /*
            IMPORT_P FOREIGN SCHEMA ColId import_qualification FROM SERVER ColId INTO ColId create_generic_options
        */

        if self.buffer.consume_kw_eq(Import)?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Foreign).required()?;
        self.buffer.consume_kw_eq(Schema).required()?;

        todo!()
    }
}

use crate::lexer::Keyword::{Foreign, Import, Schema};
use crate::parser::result::OptionalResult;
use crate::parser::AstNode;
use crate::parser::OptResult;
use crate::parser::Parser;
