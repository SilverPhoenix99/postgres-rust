/// Aliases:
/// * `CreateGroupStmt`
/// * `CreateRoleStmt`
pub fn create_role_stmt(ctx: &mut ParserContext) -> scan::Result<CreateRoleStmt> {

    /*
        CREATE (ROLE | GROUP) RoleId ( WITH )? OptRoleList
    */

    let (kind, name, _, options) = seq!(
        role_kind,
        role_id,
        With.optional(),
        create_role_options.optional()
    ).parse(ctx)?;

    let mut stmt = CreateRoleStmt::new(name, kind);
    stmt.set_options(options);

    Ok(stmt)
}

fn role_kind(ctx: &mut ParserContext) -> scan::Result<RoleKind> {

    alt!(
        Group.map(|_| RoleKind::Group),
        Role.map(|_| RoleKind::Role)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_role_ast::CreateRoleOption,
        pg_sink_ast::RoleSpec::Public,
    };

    #[test_case("role foo" => Ok(CreateRoleStmt::new("foo", RoleKind::Role)))]
    #[test_case("role test_role with sysid 42" => Ok(
        CreateRoleStmt::new("test_role", RoleKind::Role)
            .with_options(vec![CreateRoleOption::SysId(42.into())])
    ))]
    #[test_case("group test_role inherit role public" => Ok(
        CreateRoleStmt::new("test_role", RoleKind::Group)
            .with_options(vec![CreateRoleOption::AddRoleTo(vec![Public])])
    ))]
    fn test_create_role_stmt(source: &str) -> scan::Result<CreateRoleStmt> {
        test_parser!(source, create_role_stmt)
    }

    #[test_case("group" => Ok(RoleKind::Group))]
    #[test_case("role" => Ok(RoleKind::Role))]
    fn test_role_kind(source: &str) -> scan::Result<RoleKind> {
        test_parser!(source, role_kind)
    }
}

use crate::create::create_role_options;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Role;
use pg_lexer::Keyword::With;
use pg_parser_core::scan;
use pg_role_ast::CreateRoleStmt;
use pg_role_ast::RoleKind;
use pg_sink_combinators::role_id;
