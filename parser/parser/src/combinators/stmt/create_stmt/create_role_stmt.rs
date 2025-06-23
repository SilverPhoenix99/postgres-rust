/// Aliases:
/// * `CreateGroupStmt`
/// * `CreateRoleStmt`
pub(super) fn create_role_stmt() -> impl Combinator<Output = CreateRoleStmt> {

    (
        role_kind(),
        role_id,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        CreateRoleOption,
        RoleSpec::Public
    };
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
}

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_id;
use crate::combinators::stmt::create_stmt::create_role_options;
use pg_ast::CreateRoleStmt;
use pg_ast::RoleKind;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Role;
use pg_lexer::Keyword::With;
