enum Change {
    Name { new_name: Str },
    Options(Option<Vec<AlterRoleOption>>),
    Role {
        db_name: Option<Str>,
        set_stmt: SetResetClause
    }
}

pub(super) fn user_stmt(ctx: &mut ParserContext) -> scan::Result<RoleStmt> {

    /*
          ALL ( in_database )? SetResetClause      => AlterRoleSetStmt
        | RoleId RENAME TO RoleId                  => RenameStmt
        | RoleSpec ( in_database )? SetResetClause => AlterRoleSetStmt
        | RoleSpec ( WITH )? AlterOptRoleList      => AlterRoleStmt
    */

    alt!(
        seq!(All, in_database.optional(), set_reset_clause)
            .map(|(_, db_name, set_stmt)| {
                let mut stmt = AlterRoleSetStmt::new(OneOrAll::All, set_stmt);
                stmt.set_database(db_name);
                stmt.into()
            }),
        user_role_stmt
    ).parse(ctx)
}

fn user_role_stmt(ctx: &mut ParserContext) -> scan::Result<RoleStmt> {

    /*
          RoleId RENAME TO RoleId                  => RenameStmt
        | RoleSpec ( in_database )? SetResetClause => AlterRoleSetStmt
        | RoleSpec ( WITH )? AlterOptRoleList      => AlterRoleStmt
    */

    let (Located(role, loc), stmt) = seq!(
        located!(role_spec),
        alt!(
            rename,
            change_role,
            seq!(
                With.optional(),
                alter_role_options.optional()
            ).map(|(_, options)| Change::Options(options)),
        )
    ).parse(ctx)?;

    let stmt = match stmt {

        Change::Name { new_name } => {

            let role_name = role.into_role_id()
                .map_err(|err| err.at_location(loc))?;

            RoleStmt::Rename { role_name, new_name }
        },

        Change::Options(options) => {
            AlterRoleStmt::new(role, options).into()
        },

        Change::Role { db_name, set_stmt } => {

            let mut stmt = AlterRoleSetStmt::new(OneOrAll::One(role), set_stmt);
            stmt.set_database(db_name);

            stmt.into()
        },
    };

    Ok(stmt)
}

fn rename(ctx: &mut ParserContext) -> scan::Result<Change> {

    let (.., new_name) = seq!(Rename, To, role_id).parse(ctx)?;
    Ok(Change::Name { new_name })
}

fn change_role(ctx: &mut ParserContext) -> scan::Result<Change> {

    /*
        ( WITH )? AlterOptRoleList
    */

    let (db_name, set_stmt) = seq!(in_database.optional(), set_reset_clause)
        .parse(ctx)?;

    Ok(Change::Role { db_name, set_stmt })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_generic_set_ast::SetResetClause::Reset,
        pg_generic_set_ast::SetResetClause::Set,
        pg_generic_set_ast::SetRest::LocalTransactionCharacteristics,
        pg_generic_set_ast::SetRest::TransactionSnapshot,
        pg_generic_set_ast::VariableTarget::SessionAuthorization,
        pg_generic_set_ast::VariableTarget::TimeZone,
        pg_sink_ast::RoleSpec,
        pg_transaction_stmt_ast::TransactionMode::Deferrable,
    };

    #[test_case("all in database foo set transaction snapshot 'bar'" => Ok(
        AlterRoleSetStmt::new(
            OneOrAll::All,
            Set(TransactionSnapshot("bar".into()))
        )
        .with_database("foo")
        .into()
    ))]
    #[test_case("all set transaction deferrable" => Ok(
        AlterRoleSetStmt::new(
            OneOrAll::All,
            Set(LocalTransactionCharacteristics(vec![Deferrable]))
        ).into()
    ))]
    #[test_case("this_user rename to that_role" => Ok(
        RoleStmt::Rename {
            role_name: "this_user".into(),
            new_name: "that_role".into()
        }
    ))]
    #[test_case("current_user in database test_db reset session authorization" => Ok(
        AlterRoleSetStmt::new(
            OneOrAll::One(RoleSpec::CurrentUser),
            Reset(SessionAuthorization)
        )
        .with_database("test_db")
        .into()
    ))]
    #[test_case("public reset time zone" => Ok(
        AlterRoleSetStmt::new(
            OneOrAll::One(RoleSpec::Public),
            Reset(TimeZone)
        ).into()
    ))]
    #[test_case("public encrypted password 'abc123'" => Ok(
        AlterRoleStmt::new(
            RoleSpec::Public,
            Some(vec![AlterRoleOption::Password(Some("abc123".into()))]),
        ).into()
    ))]
    #[test_case("public with noinherit" => Ok(
        AlterRoleStmt::new(
            RoleSpec::Public,
            Some(vec![AlterRoleOption::Inherit(false)]),
        ).into()
    ))]
    #[test_case("public" => Ok(
        AlterRoleStmt::new(RoleSpec::Public, None).into()
    ))]
    #[test_case("public with" => Ok(
        AlterRoleStmt::new(RoleSpec::Public, None).into()
    ))]
    fn test_user_stmt(source: &str) -> scan::Result<RoleStmt> {
        test_parser!(source, user_stmt)
    }
}

use super::in_database::in_database;
use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::located;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_generic_set_ast::SetResetClause;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::With;
use pg_parser_core::scan;
use pg_role_ast::AlterRoleOption;
use pg_role_ast::AlterRoleSetStmt;
use pg_role_ast::AlterRoleStmt;
use pg_role_ast::RoleStmt;
use pg_role_stmt::alter_role_options;
use pg_sink_ast::OneOrAll;
use pg_sink_combinators::role_id;
use pg_sink_combinators::role_spec;
use pg_sink_combinators::IntoRoleId;
use pg_variable_stmt::set_reset_clause;
