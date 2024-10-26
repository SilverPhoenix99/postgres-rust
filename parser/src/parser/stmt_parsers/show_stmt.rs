impl Parser<'_> {
    /// Alias: `VariableShowStmt`
    pub(in crate::parser) fn show_stmt(&mut self) -> ParseResult<VariableShowStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::show_stmt";

        /*
            SHOW var_name
            SHOW TIME ZONE
            SHOW TRANSACTION ISOLATION LEVEL
            SHOW SESSION AUTHORIZATION
            SHOW ALL
        */

        let show_stmt = self.buffer
            .consume(|tok| match tok.keyword() {
                Some(Keyword::All) => Some(All),
                Some(Keyword::Session) => Some(SessionAuthorization),
                Some(Keyword::Time) => Some(TimeZone),
                Some(Keyword::Transaction) => Some(TransactionIsolation),
                _ => None
            })
            .optional()?;

        let Some(show_stmt) = show_stmt else {
            /*
                SHOW var_name
            */
            let var_name = self.var_name().required(fn_info!(FN_NAME))?;
            return Ok(Name(var_name))
        };

        match show_stmt {
            SessionAuthorization => {
                self.buffer.consume_kw_eq(Authorization).required(fn_info!(FN_NAME))?;
            },
            TransactionIsolation => {
                self.buffer.consume_kw_eq(Isolation).required(fn_info!(FN_NAME))?;
                self.buffer.consume_kw_eq(Level).required(fn_info!(FN_NAME))?;
            }
            TimeZone => {
                self.buffer.consume_kw_eq(Zone).required(fn_info!(FN_NAME))?;
            }
            _ => {}
        };

        Ok(show_stmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::VariableShowStmt::{SessionAuthorization, TimeZone, TransactionIsolation};
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_show_stmt_all() {
        let mut parser = Parser::new("all", DEFAULT_CONFIG);
        assert_eq!(Ok(All), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_session_authorization() {
        let mut parser = Parser::new("session authorization", DEFAULT_CONFIG);
        assert_eq!(Ok(SessionAuthorization), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_timezone() {
        let mut parser = Parser::new("time zone", DEFAULT_CONFIG);
        assert_eq!(Ok(TimeZone), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_transaction_isolation() {
        let mut parser = Parser::new("transaction isolation level", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionIsolation), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_var_name() {
        let mut parser = Parser::new("qualified.name", DEFAULT_CONFIG);

        let expected = vec!["qualified".into(), "name".into()];

        assert_eq!(Ok(Name(expected)), parser.show_stmt());
    }
}

use crate::lexer::Keyword::{self, Authorization, Isolation, Level, Zone};
use crate::parser::ast_node::VariableShowStmt::{self, *};
use crate::parser::result::{Optional, Required};
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::{ParseResult, Parser};
use postgres_basics::fn_info;
