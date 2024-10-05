impl Parser<'_> {
    pub(in crate::parser) fn close_stmt(&mut self) -> OptResult<OneOrAll> {

        if self.buffer.consume_kw_eq(Unreserved(Close))?.is_none() {
            return Ok(None)
        }

        if self.buffer.consume_kw_eq(Reserved(All))?.is_some() {
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
    fn test_close_all() {
        let mut parser = Parser::new("close all", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(OneOrAll::All)), parser.close_stmt());
    }

    #[test]
    fn test_close_named() {
        let mut parser = Parser::new("close abort", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(OneOrAll::Name("abort".into()))), parser.close_stmt());

        let mut parser = Parser::new("close ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(OneOrAll::Name("ident".into()))), parser.close_stmt());
    }
}

use crate::lexer::{
    Keyword::{Reserved, Unreserved},
    ReservedKeyword::All,
    UnreservedKeyword::Close,
};
use crate::parser::ast_node::OneOrAll;
use crate::parser::{
    result::OptionalResult,
    OptResult,
    Parser,
};
