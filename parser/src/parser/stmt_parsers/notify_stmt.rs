impl Parser<'_> {
    /// Alias: `NotifyStmt`
    pub(in crate::parser) fn notify_stmt(&mut self) -> ParseResult<NotifyStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::notify_stmt";

        /*
            NOTIFY ColId (',' SCONST)?
        */

        let condition_name = self.col_id().required(fn_info!(FN_NAME))?;

        if self.buffer.consume_op(Comma).optional()?.is_none() {
            /*
                NOTIFY ColId
            */
            return Ok(NotifyStmt::new(condition_name))
        }

        let payload = self.string().required(fn_info!(FN_NAME))?;

        Ok(NotifyStmt::with_payload(condition_name, payload))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_notify() {
        let mut parser = Parser::new("test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(NotifyStmt::new("test_ident".into())), parser.notify_stmt());
    }

    #[test]
    fn test_notify_with_payload() {
        let mut parser = Parser::new("test_ident, 'test-payload'", DEFAULT_CONFIG);
        let expected = NotifyStmt::with_payload("test_ident".into(), "test-payload".into());
        assert_eq!(Ok(expected), parser.notify_stmt());
    }
}

use crate::lexer::OperatorKind::Comma;
use crate::parser::ast_node::NotifyStmt;
use crate::parser::result::{Optional, Required};
use crate::parser::{ParseResult, Parser};
use postgres_basics::fn_info;
