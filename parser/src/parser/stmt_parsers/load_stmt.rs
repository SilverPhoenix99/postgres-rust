impl Parser<'_> {
    /// Alias: `LoadStmt`
    pub(in crate::parser) fn load_stmt(&mut self) -> ParseResult<Box<str>> {

        /*
            LOAD SCONST
        */

        string(fn_info!())
            .required(fn_info!())
            .parse(&mut self.buffer)
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

use crate::parser::combinators::{string, ParserFunc, ParserFuncHelpers};
use crate::parser::{ParseResult, Parser};
use postgres_basics::fn_info;
