impl Parser<'_> {
    /// Alias: `UnlistenStmt`
    pub(in crate::parser) fn unlisten_stmt(&mut self) -> ParseResult<OneOrAll> {

        /*
            UNLISTEN '*'
            UNLISTEN ColId
        */

        if self.buffer.consume_op(Mul).optional()?.is_some() {
            return Ok(OneOrAll::All)
        }

        let name = self.col_id().required(fn_info!())?;
        Ok(OneOrAll::Name(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_unlisten_all() {
        let mut parser = Parser::new("*", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::All), parser.unlisten_stmt());
    }

    #[test]
    fn test_unlisten_name() {
        let mut parser = Parser::new("test_name", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("test_name".into())), parser.unlisten_stmt());
    }
}

use crate::{
    lexer::OperatorKind::Mul,
    parser::{
        ast_node::OneOrAll,
        result::{Optional, Required},
        ParseResult,
        Parser
    }
};
use postgres_basics::fn_info;
