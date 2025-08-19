/// Alias: `OptRoleList`
pub fn create_role_options(ctx: &mut ParserContext) -> scan::Result<Vec<CreateRoleOption>> {

    many!(create_role_option).parse(ctx)
}

/// Alias: `CreateOptRoleElem`
fn create_role_option(ctx: &mut ParserContext) -> scan::Result<CreateRoleOption> {

    /*
          SYSID ICONST
        | ADMIN role_list
        | ROLE role_list
        | IN_P ROLE role_list
        | IN_P GROUP_P role_list
        | alter_role_option
    */

    alt!(
        sysid,
        admin,
        role,
        inherit,
        alter_role_option.map(CreateRoleOption::from)
    ).parse(ctx)
}

fn sysid(ctx: &mut ParserContext) -> scan::Result<CreateRoleOption> {
    let (_, id) = seq!(Sysid, integer).parse(ctx)?;
    Ok(CreateRoleOption::SysId(id))
}

fn admin(ctx: &mut ParserContext) -> scan::Result<CreateRoleOption> {
    let (_, members) = seq!(Admin, role_list).parse(ctx)?;
    Ok(CreateRoleOption::AdminMembers(members))
}

fn role(ctx: &mut ParserContext) -> scan::Result<CreateRoleOption> {
    let (_, roles) = seq!(Role, role_list).parse(ctx)?;
    Ok(CreateRoleOption::AddRoleTo(roles))
}

fn inherit(ctx: &mut ParserContext) -> scan::Result<CreateRoleOption> {

    let (.., roles) = seq!(
        Inherit,
        alt!(Role, Group),
        role_list
    ).parse(ctx)?;

    Ok(CreateRoleOption::AddRoleTo(roles))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use pg_sink_ast::RoleSpec::Public;
    use test_case::test_case;

    #[test]
    fn test_create_role_options() {
        test_parser!(
            source = "sysid 42 admin public role public inherit group public",
            parser = create_role_options,
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
        test_parser!(source, create_role_option, expected);
    }
}

use crate::alter_role_option;
use pg_combinators::alt;
use pg_combinators::integer;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Admin;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Inherit;
use pg_lexer::Keyword::Role;
use pg_lexer::Keyword::Sysid;
use pg_parser_core::scan;
use pg_role_ast::CreateRoleOption;
use pg_sink_combinators::role_list;
