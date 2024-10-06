impl Parser<'_> {
    /// Alias: `VacuumStmt`
    pub(in crate::parser) fn vacuum_stmt(&mut self) -> OptResult<AstNode> {

        /*
            VACUUM opt_full opt_freeze opt_verbose opt_analyze opt_vacuum_relation_list
            VACUUM '(' utility_option_list ')' opt_vacuum_relation_list
        */

        if self.buffer.consume_kw_eq(Vacuum)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Vacuum;
use crate::parser::AstNode;
use crate::parser::OptResult;
use crate::parser::Parser;
