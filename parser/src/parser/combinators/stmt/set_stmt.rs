/// Aliases:
/// * `ConstraintsSetStmt`
/// * `VariableSetStmt`
pub(super) fn set_stmt() -> impl Combinator<Output = RawStmt> {

    /*
          SET CONSTRAINTS constraints_set_list constraints_set_mode
        | SET LOCAL set_rest
        | SET ( SESSION )? set_rest
    */

    Set.and_right(match_first! {
        sequence!(Constraints, constraints_set_list(), constraints_set_mode())
            .map(|(_, constraints, mode)|
                ConstraintsSetStmt::new(constraints, mode)
            )
            .map(From::from),
        Local.and_right(set_rest())
            .map(VariableSetStmt::local)
            .map(From::from),
        optional(Session).and_right(set_rest())
            .map(VariableSetStmt::session)
            .map(From::from),
    })
}

fn constraints_set_list() -> impl Combinator<Output = OneOrAll<Vec<RangeVar>>> {

    /*
          ALL
        | qualified_name_list
    */

    match_first! {
        All.map(|_| OneOrAll::All),
        qualified_name_list().map(OneOrAll::One)
    }
}

fn constraints_set_mode() -> impl Combinator<Output = ConstraintsSetMode> {

    /*
          DEFERRED
        | IMMEDIATE
    */

    or(
        Kw::Immediate.map(|_| Immediate),
        Kw::Deferred.map(|_| Deferred)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::SetRest;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test]
    fn test_set_constraints() {
        let mut stream = TokenStream::new("set constraints all immediate", DEFAULT_CONFIG);
        let actual = set_stmt().parse(&mut stream);
        let expected = ConstraintsSetStmt::new(OneOrAll::All, Immediate);
        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_set_local() {
        let mut stream = TokenStream::new("set local transaction snapshot 'abc'", DEFAULT_CONFIG);
        let actual = set_stmt().parse(&mut stream);
        let expected = VariableSetStmt::local(SetRest::TransactionSnapshot("abc".into()));
        assert_eq!(Ok(expected.into()), actual);
    }

    #[test_case("set session transaction snapshot 'abc'")]
    #[test_case("set transaction snapshot 'abc'")]
    fn test_set_session(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = set_stmt().parse(&mut stream);
        let expected = VariableSetStmt::session(SetRest::TransactionSnapshot("abc".into()));
        assert_eq!(Ok(expected.into()), actual);
    }

    #[test_case("all", OneOrAll::All)]
    #[test_case("_relation", OneOrAll::One(vec![RangeVar::new("_relation".into())]))]
    fn test_constraints_set_list(source: &str, expected: OneOrAll<Vec<RangeVar>>) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = constraints_set_list().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }

    #[test_case("immediate", Immediate)]
    #[test_case("deferred", Deferred)]
    fn test_constraints_set_mode(source: &str, expected: ConstraintsSetMode) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = constraints_set_mode().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::All;
use crate::lexer::Keyword::Constraints;
use crate::lexer::Keyword::Local;
use crate::lexer::Keyword::Session;
use crate::lexer::Keyword::Set;
use crate::parser::ast_node::ConstraintsSetMode;
use crate::parser::ast_node::ConstraintsSetMode::Deferred;
use crate::parser::ast_node::ConstraintsSetMode::Immediate;
use crate::parser::ast_node::ConstraintsSetStmt;
use crate::parser::ast_node::OneOrAll;
use crate::parser::ast_node::RangeVar;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::VariableSetStmt;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::optional;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::qualified_name::qualified_name_list;
use crate::parser::combinators::stmt::set_rest::set_rest;
