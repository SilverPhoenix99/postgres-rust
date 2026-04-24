/// Aliases:
/// * `ConstraintsSetStmt`
/// * `VariableSetStmt`
pub(super) fn set_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          SET CONSTRAINTS constraints_set_list constraints_set_mode
        | SET LOCAL set_rest
        | SET ( SESSION )? set_rest
    */

    let (_, stmt) = seq!(
        Set,
        alt!(
            seq!(Constraints, constraints_set_list, constraints_set_mode)
                .map(|(_, constraints, mode)|
                    ConstraintsSetStmt::new(constraints, mode).into()
                ),
            seq!(Local, set_rest)
                .map(|(_, stmt)|
                    VariableSetStmt::local(stmt).into()
                ),
            seq!(Session.optional(), set_rest)
                .map(|(_, stmt)|
                    VariableSetStmt::session(stmt).into()
                )
        )
    ).parse(ctx)?;

    Ok(stmt)
}

fn constraints_set_list(ctx: &mut ParserContext) -> scan::Result<OneOrAll<Vec<RelationName>>> {

    /*
          ALL
        | qualified_name_list
    */

    alt!(
        All.map(|_| OneOrAll::All),
        qualified_name_list.map(OneOrAll::One)
    ).parse(ctx)
}

fn constraints_set_mode(ctx: &mut ParserContext) -> scan::Result<ConstraintsSetMode> {

    /*
          DEFERRED
        | IMMEDIATE
    */

    alt!(
        Kw::Immediate.map(|_| Immediate),
        Kw::Deferred.map(|_| Deferred)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::SetRest;
    use test_case::test_matrix;

    #[test_matrix("set constraints all immediate" => Ok(ConstraintsSetStmt::new(OneOrAll::All, Immediate).into()))]
    #[test_matrix("set local transaction snapshot 'abc'" => Ok(VariableSetStmt::local(SetRest::TransactionSnapshot("abc".into())).into()))]
    #[test_matrix(
        [
            "set session transaction snapshot 'abc'",
            "set transaction snapshot 'abc'"
        ]
        => Ok(VariableSetStmt::session(SetRest::TransactionSnapshot("abc".into())).into())
    )]
    fn test_set_session(source: &str) -> scan::Result<RawStmt> {
        test_parser!(source, set_stmt)
    }

    #[test_matrix("all" => Ok(OneOrAll::All))]
    #[test_matrix("_relation" => Ok(OneOrAll::One(vec![RelationName::new("_relation")])))]
    fn test_constraints_set_list(source: &str) -> scan::Result<OneOrAll<Vec<RelationName>>> {
        test_parser!(source, constraints_set_list)
    }

    #[test_matrix("immediate" => Ok(Immediate))]
    #[test_matrix("deferred" => Ok(Deferred))]
    fn test_constraints_set_mode(source: &str) -> scan::Result<ConstraintsSetMode> {
        test_parser!(source, constraints_set_mode)
    }
}

use super::set_rest;
use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::qualified_name_list;
use crate::seq;
use crate::ParserContext;
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
use pg_parser_core::scan;
