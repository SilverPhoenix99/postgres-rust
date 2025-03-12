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
fn def_acl_privilege_target() -> impl Combinator<Output = AclTarget> {

    match_first! {
        Tables.map(|_| AclTarget::Table),
        Functions.or(Routines).map(|_| AclTarget::Function),
        Sequences.map(|_| AclTarget::Sequence),
        Types.map(|_| AclTarget::Type),
        Schemas.map(|_| AclTarget::Schema),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::{AccessPrivilege, AclOption::*, AclTarget, DropBehavior, RoleSpec::*};
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test]
    fn test_alter_default_privileges_stmt() {
        let source = "default privileges in schema some_schema grant all on tables to public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = AlterDefaultPrivilegesStmt::new(
            vec![AclOption::Schemas(vec!["some_schema".into()])],
            GrantStmt::grant(
                AccessPrivilege::All(None),
                AclTarget::Table,
                vec![Public],
                false
            )
        );

        assert_eq!(Ok(expected), alter_default_privileges_stmt().parse(&mut stream));
    }

    #[test]
    fn test_acl_option_list() {
        let source = "in schema my_schema for role public for user current_user";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = vec![
            AclOption::Schemas(vec!["my_schema".into()]),
            Roles(vec![Public]),
            Roles(vec![CurrentUser]),
        ];

        assert_eq!(Ok(expected), def_acl_option_list().parse(&mut stream));
    }

    #[test]
    fn test_def_acl_option_in_schema() {
        let source = "in schema a,b,c";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(AclOption::Schemas(vec!["a".into(), "b".into(), "c".into()])), def_acl_option().parse(&mut stream));
    }

    #[test]
    fn test_def_acl_option_for_role() {
        let source = "for role public,current_role";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Roles(vec![Public, CurrentRole])), def_acl_option().parse(&mut stream));
    }

    #[test]
    fn test_def_acl_option_for_user() {
        let source = "for user my_user,session_user";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Roles(vec![Name("my_user".into()), SessionUser])), def_acl_option().parse(&mut stream));
    }

    #[test]
    fn test_grant_def_acl_action() {
        let source = "grant all on tables to public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = def_acl_action().parse(&mut stream);

        let expected = GrantStmt::grant(
            AccessPrivilege::All(None),
            AclTarget::Table,
            vec![Public],
            false
        );

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_grant_with_option_def_acl_action() {
        let source = "grant all privileges on tables to public with grant option";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = def_acl_action().parse(&mut stream);

        let expected = GrantStmt::grant(
            AccessPrivilege::All(None),
            AclTarget::Table,
            vec![Public],
            true
        );

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_revoke_def_acl_action() {
        let source = "revoke all privileges on tables from public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = def_acl_action().parse(&mut stream);

        let expected = GrantStmt::revoke(
            AccessPrivilege::All(None),
            AclTarget::Table,
            vec![Public],
            false,
            DropBehavior::Restrict
        );

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_revoke_grant_option_cascade_def_acl_action() {
        let source = "revoke grant option for all privileges on tables from public cascade";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = def_acl_action().parse(&mut stream);

        let expected = GrantStmt::revoke(
            AccessPrivilege::All(None),
            AclTarget::Table,
            vec![Public],
            true,
            DropBehavior::Cascade
        );

        assert_eq!(Ok(expected), actual);
    }

    #[test_case("tables", AclTarget::Table)]
    #[test_case("functions", AclTarget::Function)]
    #[test_case("sequences", AclTarget::Sequence)]
    #[test_case("routines", AclTarget::Function)]
    #[test_case("types", AclTarget::Type)]
    #[test_case("schemas", AclTarget::Schema)]
    fn test_def_acl_privilege_target(source: &str, expected: AclTarget) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), def_acl_privilege_target().parse(&mut stream));
    }
}

use crate::lexer::Keyword::DefaultKw;
use crate::lexer::Keyword::For;
use crate::lexer::Keyword::FromKw;
use crate::lexer::Keyword::Functions;
use crate::lexer::Keyword::Grant;
use crate::lexer::Keyword::In;
use crate::lexer::Keyword::On;
use crate::lexer::Keyword::OptionKw;
use crate::lexer::Keyword::Privileges;
use crate::lexer::Keyword::Revoke;
use crate::lexer::Keyword::Role;
use crate::lexer::Keyword::Routines;
use crate::lexer::Keyword::Schema;
use crate::lexer::Keyword::Schemas;
use crate::lexer::Keyword::Sequences;
use crate::lexer::Keyword::Tables;
use crate::lexer::Keyword::To;
use crate::lexer::Keyword::Types;
use crate::lexer::Keyword::User;
use crate::parser::ast_node::AclOption;
use crate::parser::ast_node::AclTarget;
use crate::parser::ast_node::AlterDefaultPrivilegesStmt;
use crate::parser::ast_node::GrantStmt;
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
