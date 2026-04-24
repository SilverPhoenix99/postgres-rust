/// Alias: `AlterDefaultPrivilegesStmt`
pub(in crate::combinators::stmt) fn alter_default_privileges_stmt(ctx: &mut ParserContext) -> scan::Result<AlterDefaultPrivilegesStmt> {

    /*
        ALTER DEFAULT PRIVILEGES DefACLOptionList DefACLAction
    */

    let (.., options, action) = seq!(
        DefaultKw,
        Privileges,
        def_acl_option_list.optional(),
        def_acl_action
    ).parse(ctx)?;

    let stmt = AlterDefaultPrivilegesStmt::new(options.unwrap_or_default(), action);
    Ok(stmt)
}

/// Alias: `DefACLOptionList`
fn def_acl_option_list(ctx: &mut ParserContext) -> scan::Result<Vec<AclOption>> {

    many!(def_acl_option).parse(ctx)
}

/// Alias: `DefACLOption`
fn def_acl_option(ctx: &mut ParserContext) -> scan::Result<AclOption> {

    /*
          IN SCHEMA name_list
        | FOR (ROLE | USER) role_list
    */

    alt!(
        seq!(In, Schema, name_list)
            .map(|(.., schemas)| AclOption::Schemas(schemas)),
        seq!(
            For,
            alt!(Role, User),
            role_list
        )
            .map(|(.., roles)| AclOption::Roles(roles))
    ).parse(ctx)
}

/// Alias: `DefACLAction`
///
/// This should match GRANT/REVOKE, except that individual target objects
/// are not mentioned, and we only allow a subset of object types.
///
fn def_acl_action(ctx: &mut ParserContext) -> scan::Result<GrantStmt> {

    /*
          GRANT privileges ON defacl_privilege_target TO grantee_list ( grant_option )?
        | REVOKE ( GRANT OPTION FOR )? privileges ON defacl_privilege_target FROM grantee_list ( drop_behavior )?
    */

    alt!(
        grant_stmt,
        revoke_stmt
    ).parse(ctx)
}

fn grant_stmt(ctx: &mut ParserContext) -> scan::Result<GrantStmt> {

    /*
        GRANT privileges ON defacl_privilege_target TO grantee_list ( with_grant_option )?
    */

    let (_, privileges, _, object_type, _, grantees, grant_option) = seq!(
        Grant,
        privileges,
        On,
        def_acl_privilege_target,
        To,
        grantee_list,
        with_grant_option.optional()
            .map(Option::unwrap_or_default)
    ).parse(ctx)?;

    let stmt = GrantStmt::grant(privileges, object_type, grantees, grant_option);
    Ok(stmt)
}

fn revoke_stmt(ctx: &mut ParserContext) -> scan::Result<GrantStmt> {

    /*
        REVOKE ( GRANT OPTION FOR )? privileges ON defacl_privilege_target FROM grantee_list ( drop_behavior )?
    */

    let (_, grant_option, privileges, _, object_type, _, grantees, drop_behavior) = seq!(
        Revoke,
        grant_option_for
            .optional()
            .map(Option::unwrap_or_default),
        privileges,
        On,
        def_acl_privilege_target,
        FromKw,
        grantee_list,
        drop_behavior.optional()
            .map(Option::unwrap_or_default)
    ).parse(ctx)?;

    let stmt = GrantStmt::revoke(privileges, object_type, grantees, grant_option, drop_behavior);
    Ok(stmt)
}

fn grant_option_for(ctx: &mut ParserContext) -> scan::Result<GrantOption> {

    /*
        GRANT OPTION FOR
    */

    let _ = seq!(Grant, OptionKw, For).parse(ctx)?;

    Ok(GrantOption::WithGrant)
}

/// Alias: `defacl_privilege_target`
fn def_acl_privilege_target(ctx: &mut ParserContext) -> scan::Result<PrivilegeDefaultsTarget> {

    alt!(
        Kw::Tables.map(|_| Tables),
        alt!(Kw::Functions, Routines).map(|_| Functions),
        Kw::Sequences.map(|_| Sequences),
        Kw::Types.map(|_| Types),
        Kw::Schemas.map(|_| Schemas),
        seq!(Kw::Large, Kw::Objects).map(|_| LargeObjects)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::AccessPrivilege;
    use pg_ast::DropBehavior;
    use pg_ast::RoleSpec::*;
    use test_case::test_matrix;

    #[test]
    fn test_alter_default_privileges_stmt() {
        test_parser! {
            source = "default privileges in schema some_schema grant all on tables to public",
            parser = alter_default_privileges_stmt,
            expected = AlterDefaultPrivilegesStmt::new(
                vec![AclOption::Schemas(vec!["some_schema".into()])],
                GrantStmt::grant(
                    AccessPrivilege::All { columns: None },
                    Tables,
                    vec![Public],
                    GrantOption::WithoutGrant
                )
            )
        }
    }

    #[test]
    fn test_acl_option_list() {
        test_parser!(
            source = "in schema my_schema for role public for user current_user",
            parser = def_acl_option_list,
            expected = vec![
                AclOption::Schemas(vec!["my_schema".into()]),
                AclOption::Roles(vec![Public]),
                AclOption::Roles(vec![CurrentUser]),
            ]
        )
    }

    #[test_matrix("in schema a,b,c" => Ok(
        AclOption::Schemas(vec![
            "a".into(),
            "b".into(),
            "c".into()
        ])
    ))]
    #[test_matrix("for role public,current_role" => Ok(
        AclOption::Roles(vec![Public, CurrentRole])
    ))]
    #[test_matrix("for user my_user,session_user" => Ok(
        AclOption::Roles(vec![Name("my_user".into()), SessionUser])
    ))]
    fn test_def_acl_option(source: &str) -> scan::Result<AclOption> {
        test_parser!(source, def_acl_option)
    }

    #[test_matrix("grant all on tables to public" => Ok(
        GrantStmt::grant(
            AccessPrivilege::All { columns: None },
            Tables,
            vec![Public],
            GrantOption::WithoutGrant
        )
    ))]
    #[test_matrix("grant all privileges on tables to public with grant option" => Ok(
        GrantStmt::grant(
            AccessPrivilege::All { columns: None },
            Tables,
            vec![Public],
            GrantOption::WithGrant
        )
    ))]
    #[test_matrix("revoke all privileges on tables from public" => Ok(
        GrantStmt::revoke(
            AccessPrivilege::All { columns: None },
            Tables,
            vec![Public],
            GrantOption::WithoutGrant,
            DropBehavior::Restrict
        )
    ))]
    #[test_matrix("revoke grant option for all privileges on tables from public cascade" => Ok(
        GrantStmt::revoke(
            AccessPrivilege::All { columns: None },
            Tables,
            vec![Public],
            GrantOption::WithGrant,
            DropBehavior::Cascade
        )
    ))]
    fn test_def_acl_action(source: &str) -> scan::Result<GrantStmt> {
        test_parser!(source, def_acl_action)
    }

    #[test_matrix("functions" => Ok(Functions))]
    #[test_matrix("large objects" => Ok(LargeObjects))]
    #[test_matrix("routines" => Ok(Functions))]
    #[test_matrix("schemas" => Ok(Schemas))]
    #[test_matrix("sequences" => Ok(Sequences))]
    #[test_matrix("tables" => Ok(Tables))]
    #[test_matrix("types" => Ok(Types))]
    fn test_def_acl_privilege_target(source: &str) -> scan::Result<PrivilegeDefaultsTarget> {
        test_parser!(source, def_acl_privilege_target)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::drop_behavior;
use crate::combinators::grantee_list;
use crate::combinators::name_list;
use crate::combinators::privileges;
use crate::combinators::role_list;
use crate::combinators::with_grant_option;
use crate::many;
use crate::seq;
use crate::ParserContext;
use pg_ast::AclOption;
use pg_ast::AlterDefaultPrivilegesStmt;
use pg_ast::GrantOption;
use pg_ast::GrantStmt;
use pg_ast::PrivilegeDefaultsTarget;
use pg_ast::PrivilegeDefaultsTarget::Functions;
use pg_ast::PrivilegeDefaultsTarget::LargeObjects;
use pg_ast::PrivilegeDefaultsTarget::Schemas;
use pg_ast::PrivilegeDefaultsTarget::Sequences;
use pg_ast::PrivilegeDefaultsTarget::Tables;
use pg_ast::PrivilegeDefaultsTarget::Types;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::Grant;
use pg_lexer::Keyword::In;
use pg_lexer::Keyword::On;
use pg_lexer::Keyword::OptionKw;
use pg_lexer::Keyword::Privileges;
use pg_lexer::Keyword::Revoke;
use pg_lexer::Keyword::Role;
use pg_lexer::Keyword::Routines;
use pg_lexer::Keyword::Schema;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::User;
use pg_parser_core::scan;
