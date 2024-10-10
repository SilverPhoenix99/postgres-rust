impl Parser<'_> {
    /// Alias: `TruncateStmt`
    pub(in crate::parser) fn truncate_stmt(&mut self) -> Result<AstNode, ScanErrorKind> {

        /*
            TRUNCATE opt_table relation_expr_list opt_restart_seqs opt_drop_behavior
        */

        self.buffer.consume_kw_eq(Truncate)?;

        todo!()
    }
}

use crate::lexer::Keyword::Truncate;
use crate::parser::result::ScanErrorKind;
use crate::parser::AstNode;
use crate::parser::Parser;
