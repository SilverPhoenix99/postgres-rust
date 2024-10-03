impl Parser<'_> {
    pub(in crate::parser) fn listen_stmt(&mut self) -> OptResult<Cow<'static, str>> {

        if self.buffer.consume_kw_eq(Unreserved(Listen))?.is_none() {
            return Ok(None)
        }

        Ok(Some(self.col_id().required()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_listen_stmt() {
        let mut parser = Parser::new("listen abort", DEFAULT_CONFIG);
        assert_eq!(Ok(Some("abort".into())), parser.listen_stmt());

        let mut parser = Parser::new("listen ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some("ident".into())), parser.listen_stmt());
    }
}

use crate::lexer::{
    Keyword::Unreserved,
    UnreservedKeyword::Listen,
};
use crate::parser::{
    result::OptionalResult,
    OptResult,
    Parser,
};
use std::borrow::Cow;
