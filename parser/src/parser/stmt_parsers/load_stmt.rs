impl Parser<'_> {
    /// Alias: `LoadStmt`
    pub(in crate::parser) fn load_stmt(&mut self) -> ParseResult<String> {

        /*
            LOAD SCONST
        */

        self.string().required()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_load_stmt() {
        let mut parser = Parser::new("'test string'", DEFAULT_CONFIG);
        assert_eq!(Ok("test string".into()), parser.load_stmt());
    }
}

use crate::parser::result::ScanResultTrait;
use crate::parser::{ParseResult, Parser};
