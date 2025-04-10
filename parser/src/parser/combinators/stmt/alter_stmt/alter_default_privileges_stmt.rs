/// Alias: `AlterDefaultPrivilegesStmt`
pub(super) fn alter_default_privileges_stmt() -> impl Combinator<Output = AlterDefaultPrivilegesStmt> {

    /*
        ALTER DEFAULT PRIVILEGES DefACLOptionList DefACLAction
    */

    sequence!(
        DefaultKw.and(Privileges).skip(),
        def_acl_option_list().optional(),
        def_acl_action()
    ).map(|(_, options, action)|
        AlterDefaultPrivilegesStmt::new(options.unwrap_or_default(), action)
    )
}

/// Post-condition: Vec is **Not** empty
///
/// Alias: `DefACLOptionList`
fn def_acl_option_list() -> impl Combinator<Output = Vec<AclOption>> {

    many(def_acl_option())
}

/// Alias: `DefACLOption`
fn def_acl_option() -> impl Combinator<Output = AclOption> {

    /*
          IN SCHEMA name_list
        | FOR (ROLE | USER) role_list
    */

    match_first!{

        sequence!(
            In.and(Schema),
            name_list()
        ).map(|(_, schemas)|
            AclOption::Schemas(schemas)
        ),

        sequence!(
            For.and(Role.or(User))
                .skip(),
            role_list()
        ).map(|(_, roles)|
            AclOption::Roles(roles)
        )
    }
}

/// Alias: `DefACLAction`
///
/// This should match GRANT/REVOKE, except that individual target objects
/// are not mentioned, and we only allow a subset of object types.
///
fn def_acl_action() -> impl Combinator<Output = GrantStmt> {

    /*
          GRANT privileges ON defacl_privilege_target TO grantee_list opt_grant_option
        | REVOKE ( GRANT OPTION FOR )? privileges ON defacl_privilege_target FROM grantee_list opt_drop_behavior
    */

    match_first! {
        {
            let grant = sequence!{
                Grant.and_right(privileges()),
                On.and_right(def_acl_privilege_target()),
                To.and_right(grantee_list()),
                opt_grant_option()
            };

            grant.map(|(privileges, object_type, grantees, grant_option)|
                GrantStmt::grant(privileges, object_type, grantees, grant_option)
            )
        },
        {
            let revoke = sequence!{
                Revoke.skip(),
                Grant.and(OptionKw).and(For)
                    .optional()
                    .map(|grant_option| grant_option.is_some()),
                privileges(),
                On.and_right(def_acl_privilege_target()),
                FromKw.and_right(grantee_list()),
                opt_drop_behavior()
            };

            revoke.map(|(_, grant_option, privileges, object_type, grantees, drop_behavior)|
                GrantStmt::revoke(privileges, object_type, grantees, grant_option, drop_behavior)
            )
        }
    }
}

/// Alias: `defacl_privilege_target`
fn def_acl_privilege_target() -> impl Combinator<Output = PrivilegeDefaultsTarget> {

    match_first! {
        Kw::Tables.map(|_| Tables),
        Kw::Functions.or(Routines).map(|_| Functions),
        Kw::Sequences.map(|_| Sequences),
        Kw::Types.map(|_| Types),
        Kw::Schemas.map(|_| Schemas),
        and(Kw::Large, Kw::Objects).map(|_| LargeObjects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::AccessPrivilege;
    use crate::parser::ast_node::DropBehavior;
    use crate::parser::ast_node::PrivilegeDefaultsTarget;
    use crate::parser::ast_node::RoleSpec::*;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test]
    fn test_alter_default_privileges_stmt() {
        test_parser! {
            source = "default privileges in schema some_schema grant all on tables to public",
            parser = alter_default_privileges_stmt(),
            expected = AlterDefaultPrivilegesStmt::new(
                vec![AclOption::Schemas(vec!["some_schema".into()])],
                GrantStmt::grant(
                    AccessPrivilege::All(None),
                    Tables,
                    vec![Public],
                    false
                )
            )
        }
    }

    #[test]
    fn test_acl_option_list() {
        test_parser! {
            source = "in schema my_schema for role public for user current_user",
            parser = def_acl_option_list(),
            expected = vec![
                AclOption::Schemas(vec!["my_schema".into()]),
                AclOption::Roles(vec![Public]),
                AclOption::Roles(vec![CurrentUser]),
            ]
        }
    }

    #[test]
    fn test_def_acl_option_in_schema() {
        test_parser! {
            source = "in schema a,b,c",
            parser = def_acl_option(),
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
            parser = def_acl_option(),
            expected = AclOption::Roles(vec![Public, CurrentRole])
        }
    }

    #[test]
    fn test_def_acl_option_for_user() {
        test_parser! {
            source = "for user my_user,session_user",
            parser = def_acl_option(),
            expected = AclOption::Roles(vec![Name("my_user".into()), SessionUser])
        }
    }

    #[test]
    fn test_grant_def_acl_action() {
        test_parser! {
            source = "grant all on tables to public",
            parser = def_acl_action(),
            expected = GrantStmt::grant(
                AccessPrivilege::All(None),
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
            parser = def_acl_action(),
            expected = GrantStmt::grant(
                AccessPrivilege::All(None),
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
            parser = def_acl_action(),
            expected = GrantStmt::revoke(
                AccessPrivilege::All(None),
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
            parser = def_acl_action(),
            expected = GrantStmt::revoke(
                AccessPrivilege::All(None),
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
        test_parser!(source, def_acl_privilege_target(), expected);
    }
}

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::DefaultKw;
use crate::lexer::Keyword::For;
use crate::lexer::Keyword::FromKw;
use crate::lexer::Keyword::Grant;
use crate::lexer::Keyword::In;
use crate::lexer::Keyword::On;
use crate::lexer::Keyword::OptionKw;
use crate::lexer::Keyword::Privileges;
use crate::lexer::Keyword::Revoke;
use crate::lexer::Keyword::Role;
use crate::lexer::Keyword::Routines;
use crate::lexer::Keyword::Schema;
use crate::lexer::Keyword::To;
use crate::lexer::Keyword::User;
use crate::parser::ast_node::AclOption;
use crate::parser::ast_node::AlterDefaultPrivilegesStmt;
use crate::parser::ast_node::GrantStmt;
use crate::parser::ast_node::PrivilegeDefaultsTarget;
use crate::parser::ast_node::PrivilegeDefaultsTarget::Functions;
use crate::parser::ast_node::PrivilegeDefaultsTarget::LargeObjects;
use crate::parser::ast_node::PrivilegeDefaultsTarget::Schemas;
use crate::parser::ast_node::PrivilegeDefaultsTarget::Sequences;
use crate::parser::ast_node::PrivilegeDefaultsTarget::Tables;
use crate::parser::ast_node::PrivilegeDefaultsTarget::Types;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::many;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::grantee_list;
use crate::parser::combinators::name_list;
use crate::parser::combinators::opt_drop_behavior;
use crate::parser::combinators::opt_grant_option;
use crate::parser::combinators::privileges;
use crate::parser::combinators::role_list;
