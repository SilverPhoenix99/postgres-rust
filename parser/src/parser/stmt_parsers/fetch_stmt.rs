impl Parser<'_> {
    /// Alias: `FetchStmt`
    pub(in crate::parser) fn fetch_stmt(&mut self) -> ScanResult<AstNode> {

        /*
            FETCH fetch_args
            MOVE fetch_args
        */

        self.buffer.consume_kw_eq(Fetch)?;

        todo!()
    }
}

use crate::lexer::Keyword::Fetch;
use crate::parser::AstNode;
use crate::parser::Parser;
use crate::parser::result::ScanResult;
