/// Aliases:
/// * `CreateGroupStmt`
/// * `CreateRoleStmt`
pub(super) fn create_role_stmt() -> impl Combinator<Output = CreateRoleStmt> {

    sequence!(
        role_kind(),
        role_id(),
        With.optional(),
        create_role_options()
    )
        .map(|(kind, name, _, options)|
            CreateRoleStmt::new(name, kind, options)
        )
}

fn role_kind() -> impl Combinator<Output = RoleKind> {

    match_first! {
        Group.map(|_| RoleKind::Group),
        Role.map(|_| RoleKind::Role)
    }
}

/// Post-condition: Vec is **Not** empty.
fn create_role_options() -> impl Combinator<Output = Vec<CreateRoleOption>> {

    many(create_role_option())
}

fn create_role_option() -> impl Combinator<Output = CreateRoleOption> {

    /*
          SYSID ICONST
        | ADMIN role_list
        | ROLE role_list
        | IN_P ROLE role_list
        | IN_P GROUP_P role_list
        | alter_role_option
    */

    match_first! {
        Sysid
            .and_right(integer())
            .map(CreateRoleOption::SysId),
        Admin
            .and_right(role_list())
            .map(CreateRoleOption::AdminMembers),
        Role
            .and_right(role_list())
            .map(CreateRoleOption::AddRoleTo),
        Inherit
            .and(or(Role, Group))
            .and_right(role_list())
            .map(CreateRoleOption::AddRoleTo),
        alter_role_option().map(From::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec::Public;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test_case("role test_role with sysid 42",
        CreateRoleStmt::new(
            "test_role",
            RoleKind::Role,
            vec![CreateRoleOption::SysId(42.into())]
        )
    )]
    #[test_case("group test_role inherit role public",
        CreateRoleStmt::new(
            "test_role",
            RoleKind::Group,
            vec![CreateRoleOption::AddRoleTo(vec![Public])]
        )
    )]
    fn test_create_role_stmt(source: &str, expected: CreateRoleStmt) {
        test_parser!(source, create_role_stmt(), expected);
    }

    #[test_case("group", RoleKind::Group)]
    #[test_case("role", RoleKind::Role)]
    fn test_role_kind(source: &str, expected: RoleKind) {
        test_parser!(source, role_kind(), expected);
    }

    #[test]
    fn test_create_role_options() {
        test_parser!(
            source = "sysid 42 admin public role public inherit group public",
            parser = create_role_options(),
            expected = vec![
                CreateRoleOption::SysId(42.into()),
                CreateRoleOption::AdminMembers(vec![Public]),
                CreateRoleOption::AddRoleTo(vec![Public]),
                CreateRoleOption::AddRoleTo(vec![Public])
            ]
        );
    }

    #[test_case("sysid 42", CreateRoleOption::SysId(42.into()))]
    #[test_case("admin public", CreateRoleOption::AdminMembers(vec![Public]))]
    #[test_case("role public", CreateRoleOption::AddRoleTo(vec![Public]))]
    #[test_case("inherit role public", CreateRoleOption::AddRoleTo(vec![Public]))]
    #[test_case("inherit group public", CreateRoleOption::AddRoleTo(vec![Public]))]
    #[test_case("password null", CreateRoleOption::Password(None))]
    fn test_create_role_option(source: &str, expected: CreateRoleOption) {
        test_parser!(source, create_role_option(), expected);
    }
}

use crate::lexer::Keyword::Admin;
use crate::lexer::Keyword::Group;
use crate::lexer::Keyword::Inherit;
use crate::lexer::Keyword::Role;
use crate::lexer::Keyword::Sysid;
use crate::lexer::Keyword::With;
use crate::parser::ast_node::CreateRoleOption;
use crate::parser::ast_node::CreateRoleStmt;
use crate::parser::ast_node::RoleKind;
use crate::parser::combinators::foundation::integer;
use crate::parser::combinators::foundation::many;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::role_id;
use crate::parser::combinators::role_list;
use crate::parser::combinators::stmt::alter_role_option;
