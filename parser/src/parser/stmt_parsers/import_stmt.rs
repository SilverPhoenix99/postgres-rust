impl Parser<'_> {
    /// Alias: `ImportForeignSchemaStmt`
    pub(in crate::parser) fn import_stmt(&mut self) -> Result<AstNode, ScanErrorKind> {

        /*
            IMPORT_P FOREIGN SCHEMA ColId import_qualification FROM SERVER ColId INTO ColId create_generic_options
        */

        self.buffer.consume_kw_eq(Import)?;

        self.buffer.consume_kw_eq(Foreign).required()?;
        self.buffer.consume_kw_eq(Schema).required()?;

        todo!()
    }
}

use crate::lexer::Keyword::{Foreign, Import, Schema};
use crate::parser::ast_node::AstNode;
use crate::parser::result::{ScanErrorKind, ScanResult};
use crate::parser::Parser;
