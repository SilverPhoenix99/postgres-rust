impl Parser<'_> {
    pub(in crate::parser) fn notify_stmt(&mut self) -> OptResult<NotifyStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Notify))?.is_none() {
            return Ok(None)
        }

        let condition_name = self.col_id().required()?;

        if self.buffer.consume_eq(Comma).replace_eof(Ok(None))?.is_none() {
            return Ok(Some(NotifyStmt::new(condition_name)))
        }

        let payload = self.string().required()?;

        Ok(Some(NotifyStmt::with_payload(condition_name, payload)))
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::TokenKind::Comma;
use crate::lexer::UnreservedKeyword::Notify;
use crate::parser::ast_node::NotifyStmt;
use crate::parser::result::OptionalResult;
use crate::parser::{OptResult, Parser};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_notify() {
        let mut parser = Parser::new(b"notify test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(NotifyStmt::new("test_ident".into()))), parser.notify_stmt());
    }

    #[test]
    fn test_notify_with_payload() {
        let mut parser = Parser::new(b"notify test_ident, 'test-payload'", DEFAULT_CONFIG);
        let expected = NotifyStmt::with_payload("test_ident".into(), "test-payload".into());
        assert_eq!(Ok(Some(expected)), parser.notify_stmt());
    }
}
