impl Parser<'_> {
    /// Alias: `VariableShowStmt`
    pub(in crate::parser) fn show_stmt(&mut self) -> OptResult<VariableShowStmt> {

        /*
            SHOW var_name
            SHOW TIME ZONE
            SHOW TRANSACTION ISOLATION LEVEL
            SHOW SESSION AUTHORIZATION
            SHOW ALL
        */

        if self.buffer.consume_kw_eq(Show)?.is_none() {
            return Ok(None)
        }

        let show_stmt = self.buffer.consume(|tok|
            tok.keyword().and_then(|kw| match kw.keyword() {
                All => Some(VariableShowStmt::All),
                Session => Some(VariableShowStmt::SessionAuthorization),
                Time => Some(VariableShowStmt::TimeZone),
                Transaction => Some(VariableShowStmt::TransactionIsolation),
                _ => None
            })
        ).replace_eof(Ok(None))?;

        match show_stmt {
            Some(VariableShowStmt::All) => Ok(Some(VariableShowStmt::All)),
            Some(VariableShowStmt::SessionAuthorization) => {
                self.buffer.consume_kw_eq(Authorization).required()?;
                Ok(Some(VariableShowStmt::SessionAuthorization))
            },
            Some(VariableShowStmt::TransactionIsolation) => {
                self.buffer.consume_kw_eq(Isolation).required()?;
                self.buffer.consume_kw_eq(Level).required()?;
                Ok(Some(VariableShowStmt::TransactionIsolation))
            }
            Some(VariableShowStmt::TimeZone) => {
                self.buffer.consume_kw_eq(Zone).required()?;
                Ok(Some(VariableShowStmt::TimeZone))
            }
            Some(VariableShowStmt::Name(_)) => unreachable!(),
            None => {
                let var_name = self.var_name()?;
                Ok(Some(VariableShowStmt::Name(var_name)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_show_stmt_all() {
        let mut parser = Parser::new("show all", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(VariableShowStmt::All)), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_session_authorization() {
        let mut parser = Parser::new("show session authorization", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(VariableShowStmt::SessionAuthorization)), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_timezone() {
        let mut parser = Parser::new("show time zone", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(VariableShowStmt::TimeZone)), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_transaction_isolation() {
        let mut parser = Parser::new("show transaction isolation level", DEFAULT_CONFIG);
        assert_eq!(Ok(Some(VariableShowStmt::TransactionIsolation)), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_var_name() {
        let mut parser = Parser::new("show qualified.name", DEFAULT_CONFIG);

        let expected = vec!["qualified".into(), "name".into()];

        assert_eq!(Ok(Some(VariableShowStmt::Name(expected))), parser.show_stmt());
    }
}

use crate::lexer::Keyword::{All, Authorization, Isolation, Level, Session, Show, Time, Transaction, Zone};
use crate::parser::ast_node::VariableShowStmt;
use crate::parser::result::OptionalResult;
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::OptResult;
use crate::parser::Parser;
