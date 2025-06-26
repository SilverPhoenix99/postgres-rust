/// Alias: `reset_rest`
pub(super) fn variable_target(stream: &mut TokenStream) -> scan::Result<VariableTarget> {

    /*
          TIME ZONE
        | SESSION AUTHORIZATION
        | TRANSACTION ISOLATION LEVEL
        | all_or_var_name
    */

    choice!(stream =>
        seq!(stream => Time, Zone)
            .map(|_| TimeZone),
        seq!(stream => Transaction, Isolation, Level)
            .map(|_| TransactionIsolation),
        seq!(stream => Session, Authorization)
            .map(|_| SessionAuthorization),
        all_or_var_name(stream)
            .map(|reset| match reset {
                OneOrAll::All => VariableTarget::All,
                OneOrAll::One(name) => VariableTarget::Variable{ name }
            })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("all", VariableTarget::All)]
    #[test_case("session authorization", VariableTarget::SessionAuthorization)]
    #[test_case("time zone", VariableTarget::TimeZone)]
    #[test_case("transaction isolation level", VariableTarget::TransactionIsolation)]
    #[test_case("qualified.name", VariableTarget::Variable { name: vec!["qualified".into(), "name".into()] })]
    fn test_show_stmt(source: &str, expected: VariableTarget) {
        test_parser!(source, variable_target, expected)
    }
}

use crate::combinators::all_or_var_name;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::scan;
use crate::stream::TokenStream;
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
