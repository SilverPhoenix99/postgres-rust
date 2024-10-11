impl Parser<'_> {
    /// Alias: `VacuumStmt`
    pub(in crate::parser) fn vacuum_stmt(&mut self) -> ScanResult<RawStmt> {

        /*
            VACUUM opt_full opt_freeze opt_verbose opt_analyze opt_vacuum_relation_list
            VACUUM '(' utility_option_list ')' opt_vacuum_relation_list
        */

        self.buffer.consume_kw_eq(Vacuum)?;

        todo!()
    }
}

use crate::lexer::Keyword::Vacuum;
use crate::parser::ast_node::RawStmt;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
