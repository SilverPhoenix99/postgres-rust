impl Parser<'_> {
    /// Alias: `AnalyzeStmt`
    pub(in crate::parser) fn analyze_stmt(&mut self) -> ScanResult<AstNode> {

        /*
            (ANALYSE | ANALYZE) '(' utility_option_list ')' opt_vacuum_relation_list
            (ANALYSE | ANALYZE) (VERBOSE)? opt_vacuum_relation_list
        */

        self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword).filter(|kw|
                matches!(kw, Analyse | Analyze)
            )
        )?;

        todo!()
    }
}

use crate::lexer::Keyword::{Analyse, Analyze};
use crate::lexer::KeywordDetails;
use crate::parser::ast_node::AstNode;
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::Parser;
use crate::parser::result::ScanResult;
