impl Parser<'_> {
    /// Alias: `ImportForeignSchemaStmt`
    pub(in crate::parser) fn import_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            IMPORT_P FOREIGN SCHEMA ColId import_qualification FROM SERVER ColId INTO ColId create_generic_options
        */

        self.buffer.consume_kw_eq(Foreign).required(fn_info!())?;
        self.buffer.consume_kw_eq(Schema).required(fn_info!())?;

        todo!()
    }
}

use crate::{
    lexer::Keyword::{Foreign, Schema},
    parser::{
        ast_node::RawStmt,
        result::Required,
        ParseResult,
        Parser
    }
};
use postgres_basics::fn_info;
