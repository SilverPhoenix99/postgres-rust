impl Parser<'_> {
    /// Alias: `AnalyzeStmt`
    pub(in crate::parser) fn analyze_stmt(&mut self) -> OptResult<AstNode> {

        /*
            (ANALYSE | ANALYZE) '(' utility_option_list ')' opt_vacuum_relation_list
            (ANALYSE | ANALYZE) (VERBOSE)? opt_vacuum_relation_list
        */

        let analyze = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword).filter(|kw|
                matches!(kw, Analyse | Analyze)
            )
        )?;

        if analyze.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::Keyword::{Analyse, Analyze};
use crate::lexer::KeywordDetails;
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::AstNode;
use crate::parser::OptResult;
use crate::parser::Parser;
