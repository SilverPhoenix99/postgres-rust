impl Parser<'_> {
    pub(in crate::parser) fn set_stmt(&mut self) -> ScanResult<RawStmt> {

        // TODO Conflicts

        self.buffer.consume_kw_eq(Set)?;

        todo!()
    }
}

use crate::lexer::Keyword::Set;
use crate::parser::ast_node::RawStmt;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
