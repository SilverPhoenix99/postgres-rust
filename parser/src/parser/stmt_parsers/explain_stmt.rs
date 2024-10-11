impl Parser<'_> {
    /// Alias: `ExplainStmt`
    pub(in crate::parser) fn explain_stmt(&mut self) -> ScanResult<RawStmt> {

        /*
            EXPLAIN ExplainableStmt
            EXPLAIN analyze_keyword opt_verbose ExplainableStmt
            EXPLAIN VERBOSE ExplainableStmt
            EXPLAIN '(' utility_option_list ')' ExplainableStmt
        */

        self.buffer.consume_kw_eq(Explain)?;

        todo!()
    }
}

use crate::lexer::Keyword::Explain;
use crate::parser::ast_node::RawStmt;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
