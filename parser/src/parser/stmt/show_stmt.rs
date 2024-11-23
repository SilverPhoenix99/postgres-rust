/// Alias: `VariableShowStmt`
pub(in crate::parser) fn show_stmt() -> impl Combinator<Output = VariableShowStmt> {

    /*
        SHOW ALL
        SHOW TIME ZONE
        SHOW SESSION AUTHORIZATION
        SHOW TRANSACTION ISOLATION LEVEL
        SHOW var_name
    */

    keyword(Show)
        .and_right(match_first!{
            keyword(All).skip()
                .map(|_| VariableShowStmt::All),
            keyword(Time).skip()
                .and(keyword(Zone).skip())
                .map(|_| TimeZone),
            keyword(Session).skip()
                .and(keyword(Authorization).skip())
                .map(|_| SessionAuthorization),
            keyword(Transaction).skip()
                .and(keyword(Isolation).skip())
                .and(keyword(Level).skip())
                .map(|_| TransactionIsolation),
            var_name()
                .map(Name)
        })

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::VariableShowStmt::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("show all", VariableShowStmt::All)]
    #[test_case("show session authorization", SessionAuthorization)]
    #[test_case("show time zone", TimeZone)]
    #[test_case("show transaction isolation level", TransactionIsolation)]
    #[test_case("show qualified.name", Name(vec!["qualified".into(), "name".into()]))]
    fn test_show_stmt(source: &str, expected: VariableShowStmt) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), show_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::{All, Authorization, Isolation, Level, Session, Show, Time, Transaction, Zone};
use crate::parser::ast_node::VariableShowStmt;
use crate::parser::combinators::{keyword, match_first};
use crate::parser::combinators::{Combinator, CombinatorHelpers};
use crate::parser::var_name;
use VariableShowStmt::{Name, SessionAuthorization, TimeZone, TransactionIsolation};
