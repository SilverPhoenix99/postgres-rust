impl Parser<'_> {
    /// Alias: `ListenStmt`
    pub(in crate::parser) fn listen_stmt(&mut self) -> ScanResult<CowStr> {

        /*
            LISTEN ColId
        */

        self.buffer.consume_kw_eq(Listen)?;

        Ok(self.col_id().required()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_listen_stmt() {
        let mut parser = Parser::new("listen abort", DEFAULT_CONFIG);
        assert_eq!(Ok("abort".into()), parser.listen_stmt());

        let mut parser = Parser::new("listen ident", DEFAULT_CONFIG);
        assert_eq!(Ok("ident".into()), parser.listen_stmt());
    }
}

use crate::lexer::Keyword::Listen;
use crate::parser::result::{ScanResult, ScanResultTrait};
use crate::parser::{CowStr, Parser};
