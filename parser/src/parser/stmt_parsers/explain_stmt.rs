impl Parser<'_> {
    /// Alias: `ExplainStmt`
    pub(in crate::parser) fn explain_stmt(&mut self) -> OptResult<AstNode> {

        /*
            EXPLAIN ExplainableStmt
            EXPLAIN analyze_keyword opt_verbose ExplainableStmt
            EXPLAIN VERBOSE ExplainableStmt
            EXPLAIN '(' utility_option_list ')' ExplainableStmt
        */

        if self.buffer.consume_kw_eq(Explain)?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::Explain;
use crate::parser::AstNode;
use crate::parser::OptResult;
use crate::parser::Parser;
