impl Parser<'_> {
    /// Alias: `DeallocateStmt`
    pub(in crate::parser) fn deallocate_stmt(&mut self) -> OptResult<OneOrAll> {

        /*
            DEALLOCATE (PREPARE)? ALL
            DEALLOCATE (PREPARE)? ColId
        */

        if self.buffer.consume_kw_eq(Deallocate)?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Prepare).replace_eof(Ok(None))?;

        if self.buffer.consume_kw_eq(All)?.is_some() {
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
    fn test_deallocate_all() {
        let mut parser = Parser::new("deallocate all", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(OneOrAll::All)), parser.deallocate_stmt());
    }

    #[test]
    fn test_deallocate_named() {
        let mut parser = Parser::new("deallocate abort", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(OneOrAll::Name("abort".into()))), parser.deallocate_stmt());
        let mut parser = Parser::new("deallocate ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(OneOrAll::Name("ident".into()))), parser.deallocate_stmt());
    }
}

use crate::lexer::Keyword::{All, Deallocate, Prepare};
use crate::parser::ast_node::OneOrAll;
use crate::parser::result::OptionalResult;
use crate::parser::OptResult;
use crate::parser::Parser;
