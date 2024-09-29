impl Parser<'_> {
    pub(in crate::parser) fn listen_stmt(&mut self) -> OptResult<Cow<'static, str>> {

        if self.buffer.consume_kw_eq(Unreserved(Listen))?.is_none() {
            return Ok(None)
        }

        Ok(Some(self.col_id().required()?))
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Listen;
use crate::parser::result::OptionalResult;
use crate::parser::{OptResult, Parser};
use std::borrow::Cow;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_listen_stmt() {
        let mut parser = Parser::new(b"listen abort", DEFAULT_CONFIG);
        assert_eq!(Ok(Some("abort".into())), parser.listen_stmt());

        let mut parser = Parser::new(b"listen ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some("ident".into())), parser.listen_stmt());
    }
}
