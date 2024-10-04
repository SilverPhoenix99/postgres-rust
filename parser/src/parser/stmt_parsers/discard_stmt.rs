impl Parser<'_> {
    pub(in crate::parser) fn discard_stmt(&mut self) -> OptResult<DiscardStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Discard))?.is_none() {
            return Ok(None)
        }

        let discard = self.buffer.consume(|tok| {
            match tok.keyword().map(KeywordDetails::keyword) {
                Some(Reserved(All)) => Some(DiscardStmt::All),
                Some(Unreserved(Plans)) => Some(DiscardStmt::Plans),
                Some(Unreserved(Sequences)) => Some(DiscardStmt::Sequences),
                Some(Unreserved(Temp | Temporary)) => Some(DiscardStmt::Temporary),
                _ => None,
            }
        }).required()?;

        Ok(Some(discard))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_discard_all() {
        let mut parser = Parser::new("discard all", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(DiscardStmt::All)), parser.discard_stmt());
    }

    #[test]
    fn test_discard_plans() {
        let mut parser = Parser::new("discard plans", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(DiscardStmt::Plans)), parser.discard_stmt());
    }

    #[test]
    fn test_discard_sequences() {
        let mut parser = Parser::new("discard sequences", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(DiscardStmt::Sequences)), parser.discard_stmt());
    }

    #[test]
    fn test_discard_temporary() {
        let mut parser = Parser::new("discard temp", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(DiscardStmt::Temporary)), parser.discard_stmt());
        let mut parser = Parser::new("discard temporary", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(DiscardStmt::Temporary)), parser.discard_stmt());
    }
}

use crate::lexer::Keyword::Reserved;
use crate::lexer::Keyword::Unreserved;
use crate::lexer::KeywordDetails;
use crate::lexer::ReservedKeyword::All;
use crate::lexer::UnreservedKeyword::Discard;
use crate::lexer::UnreservedKeyword::Plans;
use crate::lexer::UnreservedKeyword::Sequences;
use crate::lexer::UnreservedKeyword::Temp;
use crate::lexer::UnreservedKeyword::Temporary;
use crate::parser::ast_node::DiscardStmt;
use crate::parser::result::OptionalResult;
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::OptResult;
use crate::parser::Parser;
