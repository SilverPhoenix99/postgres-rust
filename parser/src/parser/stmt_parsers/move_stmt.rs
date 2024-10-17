impl Parser<'_> {
    /// Alias: `FetchStmt`
    pub(in crate::parser) fn move_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            FETCH fetch_args
            MOVE fetch_args
        */

        todo!()
    }
}

use crate::parser::ast_node::RawStmt;
use crate::parser::{ParseResult, Parser};
