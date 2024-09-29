impl Parser<'_> {
    pub(in crate::parser) fn load_stmt(&mut self) -> OptResult<String> {

        if self.buffer.consume_kw_eq(Unreserved(Load))?.is_none() {
            return Ok(None)
        }

        match self.string().required()? {
            StringLiteral(lit) => Ok(Some(lit)),
            _ => Err(Some(ParserErrorKind::default()))
        }
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Load;
use crate::parser::result::OptionalResult;
use crate::parser::AstLiteral::StringLiteral;
use crate::parser::{OptResult, Parser, ParserErrorKind};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_load_stmt() {
        let mut parser = Parser::new(b"load 'test string'", DEFAULT_CONFIG);
        assert_eq!(Ok(Some("test string".into())), parser.load_stmt());
    }
}
