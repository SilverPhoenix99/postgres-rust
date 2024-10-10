impl Parser<'_> {
    /// Alias: `CallStmt`
    pub(in crate::parser) fn call_stmt(&mut self) -> ScanResult<AstNode> {

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
use crate::parser::Parser;
use crate::parser::result::ScanResult;
