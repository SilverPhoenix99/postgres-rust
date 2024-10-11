impl Parser<'_> {
    /// Alias: `FetchStmt`
    pub(in crate::parser) fn move_stmt(&mut self) -> ScanResult<RawStmt> {

        /*
            FETCH fetch_args
            MOVE fetch_args
        */

        self.buffer.consume_kw_eq(Move)?;

        todo!()
    }
}

use crate::lexer::Keyword::Move;
use crate::parser::ast_node::RawStmt;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
