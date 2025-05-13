/// Alias: `reset_rest`
pub(super) fn variable_target() -> impl Combinator<Output = VariableTarget> {

    /*
          TIME ZONE
        | SESSION AUTHORIZATION
        | TRANSACTION ISOLATION LEVEL
        | all_or_var_name
    */

    match_first! {
        sequence!(Time, Zone).map(|_| TimeZone),
        sequence!(Transaction, Isolation, Level).map(|_| TransactionIsolation),
        sequence!(Session, Authorization).map(|_| SessionAuthorization),
        all_or_var_name().map(|reset| match reset {
            OneOrAll::All => VariableTarget::All,
            OneOrAll::One(name) => VariableTarget::Variable{ name }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("all", VariableTarget::All)]
    #[test_case("session authorization", VariableTarget::SessionAuthorization)]
    #[test_case("time zone", VariableTarget::TimeZone)]
    #[test_case("transaction isolation level", VariableTarget::TransactionIsolation)]
    #[test_case("qualified.name", VariableTarget::Variable { name: vec!["qualified".into(), "name".into()] })]
    fn test_show_stmt(source: &str, expected: VariableTarget) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), variable_target().parse(&mut stream));
    }
}

use crate::parser::combinators::all_or_var_name;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::OneOrAll;
use postgres_parser_ast::VariableTarget;
use postgres_parser_ast::VariableTarget::SessionAuthorization;
use postgres_parser_ast::VariableTarget::TimeZone;
use postgres_parser_ast::VariableTarget::TransactionIsolation;
use postgres_parser_lexer::Keyword::Authorization;
use postgres_parser_lexer::Keyword::Isolation;
use postgres_parser_lexer::Keyword::Level;
use postgres_parser_lexer::Keyword::Session;
use postgres_parser_lexer::Keyword::Time;
use postgres_parser_lexer::Keyword::Transaction;
use postgres_parser_lexer::Keyword::Zone;
