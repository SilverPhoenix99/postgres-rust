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
        (Constraints, constraints_set_list(), constraints_set_mode())
            .map(|(_, constraints, mode)|
                ConstraintsSetStmt::new(constraints, mode)
            )
            .map(From::from),
        Local.and_right(set_rest)
            .map(VariableSetStmt::local)
            .map(From::from),
        optional(Session).and_right(set_rest)
            .map(VariableSetStmt::session)
            .map(From::from),
    })
}

fn constraints_set_list() -> impl Combinator<Output = OneOrAll<Vec<RelationName>>> {

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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::SetRest;
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
    #[test_case("_relation", OneOrAll::One(vec![RelationName::new("_relation", None)]))]
    fn test_constraints_set_list(source: &str, expected: OneOrAll<Vec<RelationName>>) {
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

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::optional;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::qualified_name::qualified_name_list;
use crate::combinators::stmt::set_rest;
use pg_ast::ConstraintsSetMode;
use pg_ast::ConstraintsSetMode::Deferred;
use pg_ast::ConstraintsSetMode::Immediate;
use pg_ast::ConstraintsSetStmt;
use pg_ast::OneOrAll;
use pg_ast::RawStmt;
use pg_ast::RelationName;
use pg_ast::VariableSetStmt;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Constraints;
use pg_lexer::Keyword::Local;
use pg_lexer::Keyword::Session;
use pg_lexer::Keyword::Set;
