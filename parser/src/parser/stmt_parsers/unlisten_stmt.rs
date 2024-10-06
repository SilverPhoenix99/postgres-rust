impl Parser<'_> {
    /// Alias: `UnlistenStmt`
    pub(in crate::parser) fn unlisten_stmt(&mut self) -> OptResult<OneOrAll> {

        /*
            UNLISTEN '*'
            UNLISTEN ColId
        */

        if self.buffer.consume_kw_eq(Unlisten)?.is_none() {
            return Ok(None)
        }

        if self.buffer.consume_eq(Mul)?.is_some() {
            return Ok(Some(OneOrAll::All))
        }

        let name = self.col_id().required()?;
        Ok(Some(OneOrAll::Name(name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_unlisten_all() {
        let mut parser = Parser::new("unlisten *", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(OneOrAll::All)), parser.unlisten_stmt());
    }

    #[test]
    fn test_unlisten_name() {
        let mut parser = Parser::new("unlisten test_name", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(OneOrAll::Name("test_name".into()))), parser.unlisten_stmt());
    }
}

use crate::lexer::Keyword::Unlisten;
use crate::lexer::TokenKind::Mul;
use crate::parser::ast_node::OneOrAll;
use crate::parser::result::OptionalResult;
use crate::parser::OptResult;
use crate::parser::Parser;
