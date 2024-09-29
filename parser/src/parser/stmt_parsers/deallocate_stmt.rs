impl Parser<'_> {
    pub(in crate::parser) fn deallocate_stmt(&mut self) -> OptResult<DeallocateStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Deallocate))?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Unreserved(Prepare)).replace_eof(Ok(None))?;

        if self.buffer.consume_kw_eq(Reserved(All))?.is_some() {
            return Ok(Some(DeallocateStmt::All))
        }

        let name = self.col_id().required()?;
        Ok(Some(DeallocateStmt::Name(name)))
    }
}

use crate::lexer::Keyword::{Reserved, Unreserved};
use crate::lexer::ReservedKeyword::All;
use crate::lexer::UnreservedKeyword::{Deallocate, Prepare};
use crate::parser::result::OptionalResult;
use crate::parser::{DeallocateStmt, OptResult, Parser};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_deallocate_all() {
        let mut parser = Parser::new(b"deallocate all", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(DeallocateStmt::All)), parser.deallocate_stmt());
    }

    #[test]
    fn test_deallocate_named() {
        let mut parser = Parser::new(b"deallocate abort", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(DeallocateStmt::Name("abort".into()))), parser.deallocate_stmt());
        let mut parser = Parser::new(b"deallocate ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(DeallocateStmt::Name("ident".into()))), parser.deallocate_stmt());
    }
}
