impl Parser<'_> {
    pub(in crate::parser) fn close_stmt(&mut self) -> OptResult<ClosePortalStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Close))?.is_none() {
            return Ok(None)
        }

        if self.buffer.consume_kw_eq(Reserved(All))?.is_some() {
            return Ok(Some(ClosePortalStmt::All))
        }

        let name = self.col_id().required()?;
        Ok(Some(ClosePortalStmt::Name(name)))
    }
}

use crate::lexer::Keyword::{Reserved, Unreserved};
use crate::lexer::ReservedKeyword::All;
use crate::lexer::UnreservedKeyword::Close;
use crate::parser::result::OptionalResult;
use crate::parser::{ClosePortalStmt, OptResult, Parser};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_close_all() {
        let mut parser = Parser::new(b"close all", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(ClosePortalStmt::All)), parser.close_stmt());
    }

    #[test]
    fn test_close_named() {
        let mut parser = Parser::new(b"close abort", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(ClosePortalStmt::Name("abort".into()))), parser.close_stmt());

        let mut parser = Parser::new(b"close ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(ClosePortalStmt::Name("ident".into()))), parser.close_stmt());
    }
}
