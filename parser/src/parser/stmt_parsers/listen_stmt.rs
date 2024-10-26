impl Parser<'_> {
    /// Alias: `ListenStmt`
    pub(in crate::parser) fn listen_stmt(&mut self) -> ParseResult<CowStr> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::listen_stmt";

        /*
            LISTEN ColId
        */

        self.col_id().required(fn_info!(FN_NAME))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_listen_stmt() {
        let mut parser = Parser::new("abort", DEFAULT_CONFIG);
        assert_eq!(Ok("abort".into()), parser.listen_stmt());

        let mut parser = Parser::new("ident", DEFAULT_CONFIG);
        assert_eq!(Ok("ident".into()), parser.listen_stmt());
    }
}

use crate::parser::result::Required;
use crate::parser::{CowStr, ParseResult, Parser};
use postgres_basics::fn_info;
