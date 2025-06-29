/// Aliases:
/// * `ConstraintsSetStmt`
/// * `VariableSetStmt`
pub(super) fn set_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
          SET CONSTRAINTS constraints_set_list constraints_set_mode
        | SET LOCAL set_rest
        | SET ( SESSION )? set_rest
    */

    let (_, stmt) = (
        Set,
        or((
            (Constraints, constraints_set_list, constraints_set_mode)
                .map(|(_, constraints, mode)|
                    ConstraintsSetStmt::new(constraints, mode).into()
                ),
            (Local, set_rest)
                .map(|(_, stmt)|
                    VariableSetStmt::local(stmt).into()
                ),
            (Session.optional(), set_rest)
                .map(|(_, stmt)|
                    VariableSetStmt::session(stmt).into()
                )
        ))
    ).parse(stream)?;

    Ok(stmt)
}

fn constraints_set_list(stream: &mut TokenStream) -> scan::Result<OneOrAll<Vec<RelationName>>> {

    /*
          ALL
        | qualified_name_list
    */

    or((
        All.map(|_| OneOrAll::All),
        qualified_name_list.map(OneOrAll::One)
    )).parse(stream)
}

fn constraints_set_mode(stream: &mut TokenStream) -> scan::Result<ConstraintsSetMode> {

    /*
          DEFERRED
        | IMMEDIATE
    */

    or((
        Kw::Immediate.map(|_| Immediate),
        Kw::Deferred.map(|_| Deferred)
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::SetRest;
    use test_case::test_case;

    #[test]
    fn test_set_constraints() {
        test_parser!(
            source = "set constraints all immediate",
            parser = set_stmt,
            expected = ConstraintsSetStmt::new(OneOrAll::All, Immediate)
        )
    }

    #[test]
    fn test_set_local() {
        test_parser!(
            source = "set local transaction snapshot 'abc'",
            parser = set_stmt,
            expected = VariableSetStmt::local(SetRest::TransactionSnapshot("abc".into()))
        )
    }

    #[test_case("set session transaction snapshot 'abc'")]
    #[test_case("set transaction snapshot 'abc'")]
    fn test_set_session(source: &str) {
        let expected = VariableSetStmt::session(SetRest::TransactionSnapshot("abc".into()));
        test_parser!(source, set_stmt, expected)
    }

    #[test_case("all", OneOrAll::All)]
    #[test_case("_relation", OneOrAll::One(vec![RelationName::new("_relation", None)]))]
    fn test_constraints_set_list(source: &str, expected: OneOrAll<Vec<RelationName>>) {
        test_parser!(source, constraints_set_list, expected)
    }

    #[test_case("immediate", Immediate)]
    #[test_case("deferred", Deferred)]
    fn test_constraints_set_mode(source: &str, expected: ConstraintsSetMode) {
        test_parser!(source, constraints_set_mode, expected)
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::qualified_name::qualified_name_list;
use crate::combinators::stmt::set_rest;
use crate::scan;
use crate::stream::TokenStream;
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
