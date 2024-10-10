impl Parser<'_> {
    /// Alias: `FetchStmt`
    pub(in crate::parser) fn move_stmt(&mut self) -> Result<AstNode, ScanErrorKind> {

        /*
            FETCH fetch_args
            MOVE fetch_args
        */

        self.buffer.consume_kw_eq(Move)?;

        todo!()
    }
}

use crate::lexer::Keyword::Move;
use crate::parser::result::ScanErrorKind;
use crate::parser::AstNode;
use crate::parser::Parser;
