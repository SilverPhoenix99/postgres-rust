impl Parser<'_> {
    /// Alias: `ClosePortalStmt`
    pub(in crate::parser) fn close_stmt(&mut self) -> ParseResult<OneOrAll> {

        /*
            CLOSE ALL
            CLOSE ColId
        */

        if self.buffer.consume_kw_eq(All).try_match()?.is_some() {
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
    fn test_close_all() {
        let mut parser = Parser::new("all", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::All), parser.close_stmt());
    }

    #[test]
    fn test_close_named() {
        let mut parser = Parser::new("abort", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("abort".into())), parser.close_stmt());

        let mut parser = Parser::new("ident", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("ident".into())), parser.close_stmt());
    }
}

use crate::{
    lexer::Keyword::All,
    parser::{
        ast_node::OneOrAll,
        result::{Required, TryMatch},
        ParseResult,
        Parser
    }
};
