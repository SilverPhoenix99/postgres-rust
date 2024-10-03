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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::Parser;

    #[test]
    fn test_close_all() {
        let mut parser = Parser::new("close all", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(ClosePortalStmt::All)), parser.close_stmt());
    }

    #[test]
    fn test_close_named() {
        let mut parser = Parser::new("close abort", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(ClosePortalStmt::Name("abort".into()))), parser.close_stmt());

        let mut parser = Parser::new("close ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(ClosePortalStmt::Name("ident".into()))), parser.close_stmt());
    }
}

use crate::lexer::{
    Keyword::{Reserved, Unreserved},
    ReservedKeyword::All,
    UnreservedKeyword::Close,
};
use crate::parser::{
    result::OptionalResult,
    ClosePortalStmt,
    OptResult,
    Parser,
};
