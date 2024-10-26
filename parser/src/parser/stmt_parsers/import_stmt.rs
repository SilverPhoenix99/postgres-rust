impl Parser<'_> {
    /// Alias: `ImportForeignSchemaStmt`
    pub(in crate::parser) fn import_stmt(&mut self) -> ParseResult<RawStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::import_stmt";

        /*
            IMPORT_P FOREIGN SCHEMA ColId import_qualification FROM SERVER ColId INTO ColId create_generic_options
        */

        self.buffer.consume_kw_eq(Foreign).required(fn_info!(FN_NAME))?;
        self.buffer.consume_kw_eq(Schema).required(fn_info!(FN_NAME))?;

        todo!()
    }
}

use crate::lexer::Keyword::{Foreign, Schema};
use crate::parser::ast_node::RawStmt;
use crate::parser::result::Required;
use crate::parser::{ParseResult, Parser};
use postgres_basics::fn_info;
