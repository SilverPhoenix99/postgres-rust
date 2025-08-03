/// Alias: `AlterDefaultPrivilegesStmt`
pub(super) fn alter_default_privileges_stmt(stream: &mut TokenStream) -> scan::Result<AlterDefaultPrivilegesStmt> {

    /*
        ALTER DEFAULT PRIVILEGES DefACLOptionList DefACLAction
    */

    let (.., options, action) = seq!(
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

    alt!(
        seq!(In, Schema, name_list)
            .map(|(.., schemas)| AclOption::Schemas(schemas)),
        seq!(
            For,
            alt!(Role, User),
            role_list
        )
            .map(|(.., roles)| AclOption::Roles(roles))
    ).parse(stream)
}

/// Alias: `DefACLAction`
///
/// This should match GRANT/REVOKE, except that individual target objects
/// are not mentioned, and we only allow a subset of object types.
///
fn def_acl_action(stream: &mut TokenStream) -> scan::Result<GrantStmt> {

    /*
          GRANT privileges ON defacl_privilege_target TO grantee_list ( grant_option )?
        | REVOKE ( GRANT OPTION FOR )? privileges ON defacl_privilege_target FROM grantee_list ( drop_behavior )?
    */

    alt!(
        grant_stmt,
        revoke_stmt
    ).parse(stream)
}

fn grant_stmt(stream: &mut TokenStream) -> scan::Result<GrantStmt> {

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
    ).parse(stream)?;

    let stmt = GrantStmt::grant(privileges, object_type, grantees, grant_option);
    Ok(stmt)
}

fn revoke_stmt(stream: &mut TokenStream) -> scan::Result<GrantStmt> {

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
    ).parse(stream)?;

    let stmt = GrantStmt::revoke(privileges, object_type, grantees, grant_option, drop_behavior);
    Ok(stmt)
}

fn grant_option_for(stream: &mut TokenStream) -> scan::Result<GrantOption> {

    /*
        GRANT OPTION FOR
    */

    let _ = seq!(Grant, OptionKw, For).parse(stream)?;

    Ok(GrantOption::WithGrant)
}

/// Alias: `defacl_privilege_target`
fn def_acl_privilege_target(stream: &mut TokenStream) -> scan::Result<PrivilegeDefaultsTarget> {

    alt!(
        Kw::Tables.map(|_| Tables),
        alt!(Kw::Functions, Routines).map(|_| Functions),
        Kw::Sequences.map(|_| Sequences),
        Kw::Types.map(|_| Types),
        Kw::Schemas.map(|_| Schemas),
        seq!(Kw::Large, Kw::Objects).map(|_| LargeObjects)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::AccessPrivilege;
    #[allow(unused_imports)]
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

    #[test_case("in schema a,b,c", AclOption::Schemas(vec![
        "a".into(),
        "b".into(),
        "c".into()
    ]))]
    #[test_case("for role public,current_role", AclOption::Roles(vec![Public, CurrentRole]))]
    #[test_case("for user my_user,session_user", AclOption::Roles(vec![Name("my_user".into()), SessionUser]))]
    fn test_def_acl_option(source: &str, expected: AclOption) {
        test_parser!(source, def_acl_option, expected)
    }

    #[test_case("grant all on tables to public", GrantStmt::grant(
        AccessPrivilege::All { columns: None },
        Tables,
        vec![Public],
        GrantOption::WithoutGrant
    ))]
    #[test_case("grant all privileges on tables to public with grant option", GrantStmt::grant(
        AccessPrivilege::All { columns: None },
        Tables,
        vec![Public],
        GrantOption::WithGrant
    ))]
    #[test_case("revoke all privileges on tables from public", GrantStmt::revoke(
        AccessPrivilege::All { columns: None },
        Tables,
        vec![Public],
        GrantOption::WithoutGrant,
        DropBehavior::Restrict
    ))]
    #[test_case("revoke grant option for all privileges on tables from public cascade", GrantStmt::revoke(
        AccessPrivilege::All { columns: None },
        Tables,
        vec![Public],
        GrantOption::WithGrant,
        DropBehavior::Cascade
    ))]
    fn test_def_acl_action(source: &str, expected: GrantStmt) {
        test_parser!(source, def_acl_action, expected)
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

use crate::combinators::drop_behavior;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::grantee_list;
use crate::combinators::name_list;
use crate::combinators::privileges;
use crate::combinators::role_list;
use crate::combinators::with_grant_option;
use crate::scan;
use crate::stream::TokenStream;
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
