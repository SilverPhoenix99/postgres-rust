impl Parser<'_> {
    pub(in crate::parser) fn set_stmt(&mut self) -> Result<AstNode, ScanErrorKind> {

        // TODO Conflicts

        self.buffer.consume_kw_eq(Set)?;

        todo!()
    }
}

use crate::lexer::Keyword::Set;
use crate::parser::ast_node::AstNode;
use crate::parser::result::ScanErrorKind;
use crate::parser::Parser;
