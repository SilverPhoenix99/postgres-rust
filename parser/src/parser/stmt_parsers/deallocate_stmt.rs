impl Parser<'_> {
    /// Alias: `DeallocateStmt`
    pub(in crate::parser) fn deallocate_stmt(&mut self) -> ParseResult<OneOrAll> {

        /*
            DEALLOCATE (PREPARE)? ALL
            DEALLOCATE (PREPARE)? ColId
        */

        self.buffer.consume_kw_eq(Prepare).optional()?;

        if self.buffer.eof() {
            return Err(Default::default())
        }

        if self.buffer.consume_kw_eq(All).optional()?.is_some() {
            return Ok(OneOrAll::All)
        }

        let name = self.col_id().required()?;
        Ok(OneOrAll::Name(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_deallocate_all() {
        let mut parser = Parser::new("all", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::All), parser.deallocate_stmt());
    }

    #[test]
    fn test_deallocate_named() {
        let mut parser = Parser::new("abort", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("abort".into())), parser.deallocate_stmt());
        let mut parser = Parser::new("ident", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("ident".into())), parser.deallocate_stmt());
    }
}

use crate::parser::result::Required;
use crate::{
    lexer::Keyword::{All, Prepare},
    parser::{
        ast_node::OneOrAll,
        result::Optional,
        ParseResult,
        Parser
    },
};
