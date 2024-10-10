impl Parser<'_> {
    /// Alias: `LoadStmt`
    pub(in crate::parser) fn load_stmt(&mut self) -> Result<String, ScanErrorKind> {

        /*
            LOAD SCONST
        */

        self.buffer.consume_kw_eq(Load)?;

        self.string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_load_stmt() {
        let mut parser = Parser::new("load 'test string'", DEFAULT_CONFIG);
        assert_eq!(Ok("test string".into()), parser.load_stmt());
    }
}

use crate::lexer::Keyword::Load;
use crate::parser::result::ScanErrorKind;
use crate::parser::Parser;
