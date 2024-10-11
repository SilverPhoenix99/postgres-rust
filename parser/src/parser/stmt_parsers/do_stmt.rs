impl Parser<'_> {
    /// Alias: `DoStmt`
    pub(in crate::parser) fn do_stmt(&mut self) -> ScanResult<RawStmt> {

        /*
            DO dostmt_opt_list
        */

        self.buffer.consume_kw_eq(Do)?;

        todo!()
    }
}

use crate::lexer::Keyword::Do;
use crate::parser::ast_node::RawStmt;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
