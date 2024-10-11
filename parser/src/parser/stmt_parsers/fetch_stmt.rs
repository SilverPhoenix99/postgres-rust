impl Parser<'_> {
    /// Alias: `FetchStmt`
    pub(in crate::parser) fn fetch_stmt(&mut self) -> ScanResult<RawStmt> {

        /*
            FETCH fetch_args
            MOVE fetch_args
        */

        self.buffer.consume_kw_eq(Fetch)?;

        todo!()
    }
}

use crate::lexer::Keyword::Fetch;
use crate::parser::ast_node::RawStmt;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
