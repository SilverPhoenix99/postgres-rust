impl Parser<'_> {
    /// Alias: `ImportForeignSchemaStmt`
    pub(in crate::parser) fn import_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            IMPORT_P FOREIGN SCHEMA ColId import_qualification FROM SERVER ColId INTO ColId create_generic_options
        */

        self.buffer.consume_kw_eq(Foreign).required()?;
        self.buffer.consume_kw_eq(Schema).required()?;

        todo!()
    }
}

use crate::lexer::Keyword::{Foreign, Schema};
use crate::parser::ast_node::RawStmt;
use crate::parser::result::Required;
use crate::parser::{ParseResult, Parser};
