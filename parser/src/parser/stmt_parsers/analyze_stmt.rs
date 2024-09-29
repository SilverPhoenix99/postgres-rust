impl Parser<'_> {
    pub(in crate::parser) fn analyze_stmt(&mut self) -> OptResult<AstNode> {

        let analyze = self.buffer.consume(|tok|
            tok.keyword().and_then(KeywordDetails::reserved).filter(|kw|
                matches!(kw, Analyse | Analyze)
            )
        )?;

        if analyze.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

use crate::lexer::{KeywordDetails, ReservedKeyword::{Analyse, Analyze}};
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::{AstNode, OptResult, Parser};
