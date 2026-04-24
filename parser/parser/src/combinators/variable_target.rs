/// Alias: `reset_rest`
pub(super) fn variable_target(ctx: &mut ParserContext) -> scan::Result<VariableTarget> {

    /*
          TIME ZONE
        | SESSION AUTHORIZATION
        | TRANSACTION ISOLATION LEVEL
        | all_or_var_name
    */

    alt!(
        seq!(Time, Zone)
            .map(|_| TimeZone),
        seq!(Transaction, Isolation, Level)
            .map(|_| TransactionIsolation),
        seq!(Session, Authorization)
            .map(|_| SessionAuthorization),
        all_or_var_name
            .map(|reset| match reset {
                OneOrAll::All => VariableTarget::All,
                OneOrAll::One(name) => VariableTarget::Variable { name }
            })
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("all" => Ok(VariableTarget::All))]
    #[test_matrix("session authorization" => Ok(SessionAuthorization))]
    #[test_matrix("time zone" => Ok(TimeZone))]
    #[test_matrix("transaction isolation level" => Ok(TransactionIsolation))]
    #[test_matrix("qualified.name" => Ok(VariableTarget::Variable { name: vec!["qualified".into(), "name".into()] }))]
    fn test_variable_target(source: &str) -> scan::Result<VariableTarget> {
        test_parser!(source, variable_target)
    }
}

use super::all_or_var_name;
use crate::alt;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::OneOrAll;
use pg_ast::VariableTarget;
use pg_ast::VariableTarget::SessionAuthorization;
use pg_ast::VariableTarget::TimeZone;
use pg_ast::VariableTarget::TransactionIsolation;
use pg_lexer::Keyword::Authorization;
use pg_lexer::Keyword::Isolation;
use pg_lexer::Keyword::Level;
use pg_lexer::Keyword::Session;
use pg_lexer::Keyword::Time;
use pg_lexer::Keyword::Transaction;
use pg_lexer::Keyword::Zone;
use pg_parser_core::scan;
