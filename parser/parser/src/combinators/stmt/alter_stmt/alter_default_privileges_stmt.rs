/// Alias: `AlterDefaultPrivilegesStmt`
pub(super) fn alter_default_privileges_stmt(stream: &mut TokenStream) -> scan::Result<AlterDefaultPrivilegesStmt> {

    /*
        ALTER DEFAULT PRIVILEGES DefACLOptionList DefACLAction
    */

    let (.., options, action) = (
        DefaultKw,
        Privileges,
        def_acl_option_list.optional(),
        def_acl_action
    ).parse(stream)?;

    let stmt = AlterDefaultPrivilegesStmt::new(options.unwrap_or_default(), action);
    Ok(stmt)
}

/// Alias: `DefACLOptionList`
fn def_acl_option_list(stream: &mut TokenStream) -> scan::Result<Vec<AclOption>> {

    many(def_acl_option).parse(stream)
}

/// Alias: `DefACLOption`
fn def_acl_option(stream: &mut TokenStream) -> scan::Result<AclOption> {

    /*
          IN SCHEMA name_list
        | FOR (ROLE | USER) role_list
    */

    or((
        (In, Schema, name_list)
            .map(|(.., schemas)| AclOption::Schemas(schemas)),
        (
            For,
            or((Role, User)),
            role_list
        )
            .map(|(.., roles)| AclOption::Roles(roles))
    )).parse(stream)
}

/// Alias: `DefACLAction`
///
/// This should match GRANT/REVOKE, except that individual target objects
/// are not mentioned, and we only allow a subset of object types.
///
fn def_acl_action(stream: &mut TokenStream) -> scan::Result<GrantStmt> {

    /*
          GRANT privileges ON defacl_privilege_target TO grantee_list opt_grant_option
        | REVOKE ( GRANT OPTION FOR )? privileges ON defacl_privilege_target FROM grantee_list opt_drop_behavior
    */

    or((
        grant_stmt,
        revoke_stmt
    )).parse(stream)
}

fn grant_stmt(stream: &mut TokenStream) -> scan::Result<GrantStmt> {

    let (_, privileges, _, object_type, _, grantees, grant_option) = (
        Grant,
        privileges,
        On,
        def_acl_privilege_target,
        To,
        grantee_list,
        opt_grant_option
    ).parse(stream)?;

    let stmt = GrantStmt::grant(privileges, object_type, grantees, grant_option);
    Ok(stmt)
}

fn revoke_stmt(stream: &mut TokenStream) -> scan::Result<GrantStmt> {

    let (_, grant_option, privileges, _, object_type, _, grantees, drop_behavior) = (
        Revoke,
        (Grant, OptionKw, For)
            .optional()
            .map(|grant_option| grant_option.is_some()),
        privileges,
        On,
        def_acl_privilege_target,
        FromKw,
        grantee_list,
        opt_drop_behavior
    ).parse(stream)?;

    let stmt = GrantStmt::revoke(privileges, object_type, grantees, grant_option, drop_behavior);
    Ok(stmt)
}

/// Alias: `defacl_privilege_target`
fn def_acl_privilege_target(stream: &mut TokenStream) -> scan::Result<PrivilegeDefaultsTarget> {

    or((
        Kw::Tables.map(|_| Tables),
        or((Kw::Functions, Routines)).map(|_| Functions),
        Kw::Sequences.map(|_| Sequences),
        Kw::Types.map(|_| Types),
        Kw::Schemas.map(|_| Schemas),
        (Kw::Large, Kw::Objects).map(|_| LargeObjects)
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::AccessPrivilege;
    use pg_ast::DropBehavior;
    use pg_ast::RoleSpec::*;
    use test_case::test_case;

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
                    false
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

    #[test]
    fn test_def_acl_option_in_schema() {
        test_parser! {
            source = "in schema a,b,c",
            parser = def_acl_option,
            expected = AclOption::Schemas(vec![
                "a".into(),
                "b".into(),
                "c".into()
            ])
        }
    }

    #[test]
    fn test_def_acl_option_for_role() {
        test_parser! {
            source = "for role public,current_role",
            parser = def_acl_option,
            expected = AclOption::Roles(vec![Public, CurrentRole])
        }
    }

    #[test]
    fn test_def_acl_option_for_user() {
        test_parser! {
            source = "for user my_user,session_user",
            parser = def_acl_option,
            expected = AclOption::Roles(vec![Name("my_user".into()), SessionUser])
        }
    }

    #[test]
    fn test_grant_def_acl_action() {
        test_parser! {
            source = "grant all on tables to public",
            parser = def_acl_action,
            expected = GrantStmt::grant(
                AccessPrivilege::All { columns: None },
                Tables,
                vec![Public],
                false
            )
        }
    }

    #[test]
    fn test_grant_with_option_def_acl_action() {
        test_parser! {
            source = "grant all privileges on tables to public with grant option",
            parser = def_acl_action,
            expected = GrantStmt::grant(
                AccessPrivilege::All { columns: None },
                Tables,
                vec![Public],
                true
            )
        }
    }

    #[test]
    fn test_revoke_def_acl_action() {
        test_parser! {
            source = "revoke all privileges on tables from public",
            parser = def_acl_action,
            expected = GrantStmt::revoke(
                AccessPrivilege::All { columns: None },
                Tables,
                vec![Public],
                false,
                DropBehavior::Restrict
            )
        }
    }

    #[test]
    fn test_revoke_grant_option_cascade_def_acl_action() {
        test_parser! {
            source = "revoke grant option for all privileges on tables from public cascade",
            parser = def_acl_action,
            expected = GrantStmt::revoke(
                AccessPrivilege::All { columns: None },
                Tables,
                vec![Public],
                true,
                DropBehavior::Cascade
            )
        }
    }

    #[test_case("functions", Functions)]
    #[test_case("large objects", LargeObjects)]
    #[test_case("routines", Functions)]
    #[test_case("schemas", Schemas)]
    #[test_case("sequences", Sequences)]
    #[test_case("tables", Tables)]
    #[test_case("types", Types)]
    fn test_def_acl_privilege_target(source: &str, expected: PrivilegeDefaultsTarget) {
        test_parser!(source, def_acl_privilege_target, expected);
    }
}

use crate::combinators::foundation::many;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::grantee_list;
use crate::combinators::name_list;
use crate::combinators::opt_drop_behavior;
use crate::combinators::opt_grant_option;
use crate::combinators::privileges;
use crate::combinators::role_list;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::AclOption;
use pg_ast::AlterDefaultPrivilegesStmt;
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
