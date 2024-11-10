impl Parser<'_> {
    /// Alias: `VacuumStmt`
    pub(in crate::parser) fn vacuum_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            VACUUM opt_full opt_freeze opt_verbose opt_analyze opt_vacuum_relation_list
            VACUUM '(' utility_option_list ')' opt_vacuum_relation_list
        */

        todo!()
    }
}

use crate::parser::{
    ast_node::RawStmt,
    ParseResult,
    Parser
};
