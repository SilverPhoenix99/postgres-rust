enum Change {
    Name { new_name: Str },
    Options(Option<Vec<AlterRoleOption>>),
    Role {
        db_name: Option<Str>,
        set_stmt: SetResetClause
    }
}

pub(super) fn user_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
          ALL ( in_database )? SetResetClause   => AlterRoleSetStmt
        | RoleId RENAME TO RoleId               => RenameStmt
        | RoleSpec in_database SetResetClause   => AlterRoleSetStmt
        | RoleSpec WITH AlterOptRoleList        => AlterRoleStmt
        | RoleSpec SetResetClause               => AlterRoleSetStmt
        | RoleSpec AlterOptRoleList             => AlterRoleStmt
    */

    alt!(
        seq!(All, in_database.optional(), set_reset_clause)
            .map(|(_, dbname, set_stmt)|
                AlterRoleSetStmt::new(OneOrAll::All, dbname, set_stmt).into()
            ),
        user_role_stmt
    ).parse(stream)
}

fn user_role_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
          RoleId RENAME TO RoleId               => RenameStmt
        | RoleSpec in_database SetResetClause   => AlterRoleSetStmt
        | RoleSpec WITH AlterOptRoleList        => AlterRoleStmt
        | RoleSpec SetResetClause               => AlterRoleSetStmt
        | RoleSpec AlterOptRoleList             => AlterRoleStmt
    */

    let ((role, loc), stmt) = seq!(
        located!(role_spec),
        alt!(
            rename,
            change_role,
            seq!(
                With,
                alter_role_options.optional()
            )
                .map(|(_, options)| Change::Options(options)),
            alter_role_options
                .optional()
                .map(Change::Options)
        )
    ).parse(stream)?;

    let stmt = match stmt {
        Change::Name { new_name } => {
            let role_id = role.into_role_id()
                .map_err(|err| err.at(loc))?;
            RenameStmt::new(Role(role_id), new_name).into()
        },
        Change::Options(options) => {
            AlterRoleStmt::new(role, Add, options).into()
        },
        Change::Role { db_name, set_stmt } => {
            AlterRoleSetStmt::new(OneOrAll::One(role), db_name, set_stmt).into()
        },
    };

    Ok(stmt)
}

fn rename(stream: &mut TokenStream) -> scan::Result<Change> {

    let (.., new_name) = seq!(Rename, To, role_id).parse(stream)?;
    Ok(Change::Name { new_name })
}

fn change_role(stream: &mut TokenStream) -> scan::Result<Change> {

    alt!(
        seq!(in_database, set_reset_clause)
            .map(|(db_name, set_stmt)|
                Change::Role {
                    db_name: Some(db_name),
                    set_stmt
                }
            ),
        set_reset_clause
            .map(|set_stmt|
                Change::Role {
                    db_name: None,
                    set_stmt
                }
            ),
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::{
        RoleSpec,
        SetResetClause::Reset,
        SetResetClause::Set,
        SetRest::LocalTransactionCharacteristics,
        SetRest::TransactionSnapshot,
        TransactionMode::Deferrable,
        VariableTarget::SessionAuthorization,
        VariableTarget::TimeZone,
    };
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case(
        "all in database foo set transaction snapshot 'bar'",
        AlterRoleSetStmt::new(
            OneOrAll::All,
            Some("foo".into()),
            Set(TransactionSnapshot("bar".into()))
        ).into()
    )]
    #[test_case(
        "all set transaction deferrable",
        AlterRoleSetStmt::new(
            OneOrAll::All,
            None,
            Set(LocalTransactionCharacteristics(vec![Deferrable]))
        ).into()
    )]
    #[test_case(
        "this_user rename to that_role",
        RenameStmt::new(
            Role("this_user".into()),
            "that_role"
        ).into()
    )]
    #[test_case(
        "current_user in database test_db reset session authorization",
        AlterRoleSetStmt::new(
            OneOrAll::One(RoleSpec::CurrentUser),
            Some("test_db".into()),
            Reset(SessionAuthorization)
        ).into()
    )]
    #[test_case(
        "public reset time zone",
        AlterRoleSetStmt::new(
            OneOrAll::One(RoleSpec::Public),
            None,
            Reset(TimeZone)
        ).into()
    )]
    #[test_case(
        "public encrypted password 'abc123'",
        AlterRoleStmt::new(
            RoleSpec::Public,
            Add,
            Some(vec![AlterRoleOption::Password(Some("abc123".into()))]),
        ).into()
    )]
    #[test_case(
        "public with noinherit",
        AlterRoleStmt::new(
            RoleSpec::Public,
            Add,
            Some(vec![AlterRoleOption::Inherit(false)]),
        ).into()
    )]
    #[test_case(
        "public",
        AlterRoleStmt::new(RoleSpec::Public, Add, None).into()
    )]
    #[test_case(
        "public with",
        AlterRoleStmt::new(RoleSpec::Public, Add, None).into()
    )]
    fn test_user_stmt(source: &str, expected: RawStmt) {
        test_parser!(source, user_stmt, expected)
    }
}

use super::in_database::in_database;
use crate::combinators::foundation::alt;
use crate::combinators::role_id;
use crate::combinators::role_spec;
use crate::combinators::stmt::alter_role_options;
use crate::combinators::stmt::alter_stmt::set_reset_clause;
use pg_ast::AddDrop::Add;
use pg_ast::AlterRoleOption;
use pg_ast::AlterRoleSetStmt;
use pg_ast::AlterRoleStmt;
use pg_ast::OneOrAll;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget::Role;
use pg_ast::SetResetClause;
use pg_basics::Str;
use pg_combinators::located;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::With;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
