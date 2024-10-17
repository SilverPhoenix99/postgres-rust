impl Parser<'_> {
    /// Alias: `CallStmt`
    pub(in crate::parser) fn call_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
        CallStmt:
            CALL func_application
        */

        todo!()
    }
}

use crate::parser::ast_node::RawStmt;
use crate::parser::{ParseResult, Parser};
