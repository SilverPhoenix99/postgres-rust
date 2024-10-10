impl Parser<'_> {
    /// Alias: `DeallocateStmt`
    pub(in crate::parser) fn deallocate_stmt(&mut self) -> ScanResult<OneOrAll> {

        /*
            DEALLOCATE (PREPARE)? ALL
            DEALLOCATE (PREPARE)? ColId
        */

        self.buffer.consume_kw_eq(Deallocate)?;
        self.buffer.consume_kw_eq(Prepare).optional()?;

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
    fn test_deallocate_all() {
        let mut parser = Parser::new("deallocate all", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::All), parser.deallocate_stmt());
    }

    #[test]
    fn test_deallocate_named() {
        let mut parser = Parser::new("deallocate abort", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("abort".into())), parser.deallocate_stmt());
        let mut parser = Parser::new("deallocate ident", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("ident".into())), parser.deallocate_stmt());
    }
}

use crate::lexer::Keyword::{All, Deallocate, Prepare};
use crate::parser::ast_node::OneOrAll;
use crate::parser::result::{EofResultTrait, ScanResult, ScanResultTrait};
use crate::parser::Parser;
