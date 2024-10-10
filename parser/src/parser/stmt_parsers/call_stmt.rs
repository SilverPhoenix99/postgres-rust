impl Parser<'_> {
    /// Alias: `CallStmt`
    pub(in crate::parser) fn call_stmt(&mut self) -> Result<AstNode, ScanErrorKind> {

        /*
        CallStmt:
            CALL func_application
        */

        self.buffer.consume_kw_eq(Call)?;

        todo!()
    }
}

use crate::lexer::Keyword::Call;
use crate::parser::ast_node::AstNode;
use crate::parser::result::ScanErrorKind;
use crate::parser::Parser;

