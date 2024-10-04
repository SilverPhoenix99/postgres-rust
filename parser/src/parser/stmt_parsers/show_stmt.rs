impl Parser<'_> {
    pub(in crate::parser) fn show_stmt(&mut self) -> OptResult<VariableShowStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Show))?.is_none() {
            return Ok(None)
        }

        let show_stmt = self.buffer.consume(|tok|
            tok.keyword().and_then(|kw| match kw.keyword() {
                Reserved(All) => Some(VariableShowStmt::All),
                Unreserved(Session) => Some(VariableShowStmt::SessionAuthorization),
                ColumnName(Time) => Some(VariableShowStmt::TimeZone),
                Unreserved(Transaction) => Some(VariableShowStmt::TransactionIsolation),
                _ => None
            })
        ).replace_eof(Ok(None))?;

        match show_stmt {
            Some(VariableShowStmt::All) => Ok(Some(VariableShowStmt::All)),
            Some(VariableShowStmt::SessionAuthorization) => {
                self.buffer.consume_kw_eq(TypeFuncName(Authorization)).required()?;
                Ok(Some(VariableShowStmt::SessionAuthorization))
            },
            Some(VariableShowStmt::TransactionIsolation) => {
                self.buffer.consume_kw_eq(Unreserved(Isolation)).required()?;
                self.buffer.consume_kw_eq(Unreserved(Level)).required()?;
                Ok(Some(VariableShowStmt::TransactionIsolation))
            }
            Some(VariableShowStmt::TimeZone) => {
                self.buffer.consume_kw_eq(Unreserved(Zone)).required()?;
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

use crate::lexer::ColumnNameKeyword::Time;
use crate::lexer::Keyword::ColumnName;
use crate::lexer::Keyword::Reserved;
use crate::lexer::Keyword::TypeFuncName;
use crate::lexer::Keyword::Unreserved;
use crate::lexer::ReservedKeyword::All;
use crate::lexer::TypeFuncNameKeyword::Authorization;
use crate::lexer::UnreservedKeyword::Isolation;
use crate::lexer::UnreservedKeyword::Level;
use crate::lexer::UnreservedKeyword::Session;
use crate::lexer::UnreservedKeyword::Show;
use crate::lexer::UnreservedKeyword::Transaction;
use crate::lexer::UnreservedKeyword::Zone;
use crate::parser::ast_node::VariableShowStmt;
use crate::parser::result::OptionalResult;
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::OptResult;
use crate::parser::Parser;
