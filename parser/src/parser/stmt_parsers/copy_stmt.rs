impl Parser<'_> {
    /// Alias: `CopyStmt`
    pub(in crate::parser) fn copy_stmt(&mut self) -> OptResult<AstNode> {

        /*
            COPY opt_binary qualified_name opt_column_list copy_from opt_program copy_file_name copy_delimiter opt_with copy_options where_clause
            COPY '(' PreparableStmt ')' TO opt_program copy_file_name opt_with copy_options
        */

        if self.buffer.consume_kw_eq(CopyKw)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::CopyKw;
use crate::parser::AstNode;
use crate::parser::OptResult;
use crate::parser::Parser;
