/// Alias: `reset_rest`
pub fn variable_target(ctx: &mut ParserContext) -> scan::Result<VariableTarget> {

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
    use pg_combinators::test_parser;
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

use crate::all_or_var_name;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_generic_set_ast::VariableTarget;
use pg_generic_set_ast::VariableTarget::SessionAuthorization;
use pg_generic_set_ast::VariableTarget::TimeZone;
use pg_generic_set_ast::VariableTarget::TransactionIsolation;
use pg_lexer::Keyword::Authorization;
use pg_lexer::Keyword::Isolation;
use pg_lexer::Keyword::Level;
use pg_lexer::Keyword::Session;
use pg_lexer::Keyword::Time;
use pg_lexer::Keyword::Transaction;
use pg_lexer::Keyword::Zone;
use pg_parser_core::scan;
use pg_sink_ast::OneOrAll;
