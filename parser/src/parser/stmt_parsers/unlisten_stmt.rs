impl Parser<'_> {
    /// Alias: `UnlistenStmt`
    pub(in crate::parser) fn unlisten_stmt(&mut self) -> ScanResult<OneOrAll> {

        /*
            UNLISTEN '*'
            UNLISTEN ColId
        */

        self.buffer.consume_kw_eq(Unlisten)?;

        if self.buffer.consume_eq(Mul).optional()?.is_some() {
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
    fn test_unlisten_all() {
        let mut parser = Parser::new("unlisten *", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::All), parser.unlisten_stmt());
    }

    #[test]
    fn test_unlisten_name() {
        let mut parser = Parser::new("unlisten test_name", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("test_name".into())), parser.unlisten_stmt());
    }
}

use crate::lexer::Keyword::Unlisten;
use crate::lexer::TokenKind::Mul;
use crate::parser::ast_node::OneOrAll;
use crate::parser::Parser;
use crate::parser::result::ScanResult;
use crate::parser::ScanResultTrait;
