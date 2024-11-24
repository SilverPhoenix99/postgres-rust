/// Alias: `VariableShowStmt`
pub(in crate::parser) fn show_stmt() -> impl Combinator<Output = VariableShowStmt> {

    /*
        SHOW ALL
        SHOW TIME ZONE
        SHOW SESSION AUTHORIZATION
        SHOW TRANSACTION ISOLATION LEVEL
        SHOW var_name
    */

    Show.and_right(match_first!{
        All.skip()
            .map(|_| VariableShowStmt::All),
        Time.and(Zone).skip()
            .map(|_| TimeZone),
        Session.and(Authorization).skip()
            .map(|_| SessionAuthorization),
        Transaction.and(Isolation).and(Level).skip()
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

use crate::lexer::Keyword::All;
use crate::lexer::Keyword::Authorization;
use crate::lexer::Keyword::Isolation;
use crate::lexer::Keyword::Level;
use crate::lexer::Keyword::Session;
use crate::lexer::Keyword::Show;
use crate::lexer::Keyword::Time;
use crate::lexer::Keyword::Transaction;
use crate::lexer::Keyword::Zone;
use crate::parser::ast_node::VariableShowStmt;
use crate::parser::ast_node::VariableShowStmt::Name;
use crate::parser::ast_node::VariableShowStmt::SessionAuthorization;
use crate::parser::ast_node::VariableShowStmt::TimeZone;
use crate::parser::ast_node::VariableShowStmt::TransactionIsolation;
use crate::parser::combinators::match_first;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::var_name;
