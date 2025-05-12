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

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::{
        CreateRoleOption,
        RoleSpec::Public
    };
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
}

use crate::parser::ast_node::CreateRoleStmt;
use crate::parser::ast_node::RoleKind;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::role_id;
use crate::parser::combinators::stmt::create_stmt::create_role_options;
use postgres_parser_lexer::Keyword::Group;
use postgres_parser_lexer::Keyword::Role;
use postgres_parser_lexer::Keyword::With;
