impl Parser<'_> {
    /// Alias: `VariableShowStmt`
    pub(in crate::parser) fn show_stmt(&mut self) -> Result<VariableShowStmt, ScanErrorKind> {

        /*
            SHOW var_name
            SHOW TIME ZONE
            SHOW TRANSACTION ISOLATION LEVEL
            SHOW SESSION AUTHORIZATION
            SHOW ALL
        */

        self.buffer.consume_kw_eq(Show)?;

        let show_stmt = self.buffer
            .consume(|tok|
                tok.keyword().and_then(|kw| match kw.keyword() {
                    Keyword::All => Some(All),
                    Keyword::Session => Some(SessionAuthorization),
                    Keyword::Time => Some(TimeZone),
                    Keyword::Transaction => Some(TransactionIsolation),
                    _ => None
                })
            )
            .optional()?;

        match show_stmt {
            Some(All) => Ok(All),
            Some(SessionAuthorization) => {
                self.buffer.consume_kw_eq(Authorization).required()?;
                Ok(SessionAuthorization)
            },
            Some(TransactionIsolation) => {
                self.buffer.consume_kw_eq(Isolation).required()?;
                self.buffer.consume_kw_eq(Level).required()?;
                Ok(TransactionIsolation)
            }
            Some(TimeZone) => {
                self.buffer.consume_kw_eq(Zone).required()?;
                Ok(TimeZone)
            }
            Some(Name(_)) => unreachable!(),
            None => {
                let var_name = self.var_name().required()?;
                Ok(Name(var_name))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::VariableShowStmt::{SessionAuthorization, TimeZone, TransactionIsolation};
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_show_stmt_all() {
        let mut parser = Parser::new("show all", DEFAULT_CONFIG);
        assert_eq!(Ok(All), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_session_authorization() {
        let mut parser = Parser::new("show session authorization", DEFAULT_CONFIG);
        assert_eq!(Ok(SessionAuthorization), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_timezone() {
        let mut parser = Parser::new("show time zone", DEFAULT_CONFIG);
        assert_eq!(Ok(TimeZone), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_transaction_isolation() {
        let mut parser = Parser::new("show transaction isolation level", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionIsolation), parser.show_stmt());
    }

    #[test]
    fn test_show_stmt_var_name() {
        let mut parser = Parser::new("show qualified.name", DEFAULT_CONFIG);

        let expected = vec!["qualified".into(), "name".into()];

        assert_eq!(Ok(Name(expected)), parser.show_stmt());
    }
}

use crate::lexer::Keyword::{self, Authorization, Isolation, Level, Show, Zone};
use crate::parser::ast_node::VariableShowStmt::{self, *};
use crate::parser::result::{ScanErrorKind, ScanResult};
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::Parser;
