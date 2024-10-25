impl Parser<'_> {
    /// Alias: `DiscardStmt`
    pub(in crate::parser) fn discard_stmt(&mut self) -> ParseResult<DiscardStmt> {

        /*
            DISCARD (ALL | PLANS | SEQUENCES | TEMP | TEMPORARY)
        */

        let stmt = self.buffer
            .consume(|tok| match tok.keyword() {
                Some(All) => Some(DiscardStmt::All),
                Some(Plans) => Some(DiscardStmt::Plans),
                Some(Sequences) => Some(DiscardStmt::Sequences),
                Some(Temp | Temporary) => Some(DiscardStmt::Temporary),
                _ => None,
            })
            .required()?;

        Ok(stmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::Parser;

    #[test]
    fn test_discard_all() {
        let mut parser = Parser::new("all", DEFAULT_CONFIG);
        assert_eq!(Ok(DiscardStmt::All), parser.discard_stmt());
    }

    #[test]
    fn test_discard_plans() {
        let mut parser = Parser::new("plans", DEFAULT_CONFIG);
        assert_eq!(Ok(DiscardStmt::Plans), parser.discard_stmt());
    }

    #[test]
    fn test_discard_sequences() {
        let mut parser = Parser::new("sequences", DEFAULT_CONFIG);
        assert_eq!(Ok(DiscardStmt::Sequences), parser.discard_stmt());
    }

    #[test]
    fn test_discard_temporary() {
        let mut parser = Parser::new("temp", DEFAULT_CONFIG);
        assert_eq!(Ok(DiscardStmt::Temporary), parser.discard_stmt());
        let mut parser = Parser::new("temporary", DEFAULT_CONFIG);
        assert_eq!(Ok(DiscardStmt::Temporary), parser.discard_stmt());
    }
}

use crate::lexer::Keyword::{All, Plans, Sequences, Temp, Temporary};
use crate::parser::ast_node::DiscardStmt;
use crate::parser::result::Required;
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::{ParseResult, Parser};
