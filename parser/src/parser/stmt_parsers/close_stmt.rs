impl Parser<'_> {
    /// Alias: `ClosePortalStmt`
    pub(in crate::parser) fn close_stmt(&mut self) -> ScanResult<OneOrAll> {

        /*
            CLOSE ALL
            CLOSE ColId
        */

        self.buffer.consume_kw_eq(Close)?;

        if self.buffer.consume_kw_eq(All).no_match_to_option().required()?.is_some() {
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
    fn test_close_all() {
        let mut parser = Parser::new("close all", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::All), parser.close_stmt());
    }

    #[test]
    fn test_close_named() {
        let mut parser = Parser::new("close abort", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("abort".into())), parser.close_stmt());

        let mut parser = Parser::new("close ident", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("ident".into())), parser.close_stmt());
    }
}

use crate::lexer::Keyword::{All, Close};
use crate::parser::ast_node::OneOrAll;
use crate::parser::result::{EofResultTrait, ScanResult, ScanResultTrait};
use crate::parser::Parser;
