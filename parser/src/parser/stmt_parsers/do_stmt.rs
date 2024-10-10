impl Parser<'_> {
    /// Alias: `DoStmt`
    pub(in crate::parser) fn do_stmt(&mut self) -> Result<AstNode, ScanErrorKind> {

        /*
            DO dostmt_opt_list
        */

        self.buffer.consume_kw_eq(Do)?;

        todo!()
    }
}

use crate::lexer::Keyword::Do;
use crate::parser::ast_node::AstNode;
use crate::parser::result::ScanErrorKind;
use crate::parser::Parser;

