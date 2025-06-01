/// Alias: `OptRoleList`
pub(super) fn create_role_options() -> impl Combinator<Output = Vec<CreateRoleOption>> {

    many(create_role_option())
}

/// Alias: `CreateOptRoleElem`
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
    use crate::tests::test_parser;
    use pg_ast::RoleSpec::Public;
    use test_case::test_case;

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

use crate::combinators::foundation::integer;
use crate::combinators::foundation::many;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::role::role_list;
use crate::combinators::stmt::alter_role_option;
use pg_ast::CreateRoleOption;
use pg_lexer::Keyword::Admin;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Inherit;
use pg_lexer::Keyword::Role;
use pg_lexer::Keyword::Sysid;
