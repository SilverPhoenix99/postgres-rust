impl Parser<'_> {
    /// Alias: `TruncateStmt`
    pub(in crate::parser) fn truncate_stmt(&mut self) -> OptResult<AstNode> {

        /*
            TRUNCATE opt_table relation_expr_list opt_restart_seqs opt_drop_behavior
        */
        
        if self.buffer.consume_kw_eq(Truncate)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Truncate;
use crate::parser::AstNode;
use crate::parser::OptResult;
use crate::parser::Parser;
