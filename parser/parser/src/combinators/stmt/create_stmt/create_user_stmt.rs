pub(super) fn create_user_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          USER MAPPING ( if_not_exists )? FOR auth_ident SERVER ColId create_generic_options => CreateUserMappingStmt
        | USER RoleId ( WITH )? OptRoleList => CreateRoleStmt
    */

    let (_, stmt) = seq!(
        User,
        alt!(
            create_user_mapping.map(RawStmt::from),
            create_user_role.map(RawStmt::from)
        )
    ).parse(ctx)?;

    Ok(stmt)
}

fn create_user_mapping(ctx: &mut ParserContext) -> scan::Result<CreateUserMappingStmt> {

    let (_, existence, _, user, _, server, options) = seq!(
        Mapping,
        if_not_exists.optional()
            .map(Option::unwrap_or_default),
        For,
        auth_ident,
        Server,
        col_id,
        create_generic_options.optional()
    ).parse(ctx)?;

    let stmt = CreateUserMappingStmt::new(user, server, options, existence);
    Ok(stmt)
}

fn create_user_role(ctx: &mut ParserContext) -> scan::Result<CreateRoleStmt> {

    let (name, _, options) = seq!(
        role_id,
        With.optional(),
        create_role_options
    ).parse(ctx)?;

    let stmt = CreateRoleStmt::new(name, RoleKind::User, options);
    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::CreateRoleOption;
    use pg_combinators::test_parser;
    use test_case::{test_case, test_matrix};
    #[allow(unused_imports)]
    use {
        pg_ast::GenericOption,
        pg_ast::Presence,
        pg_sink_ast::RoleSpec,
    };

    // This only quickly tests that statement types aren't missing.
    // More in-depth testing is within each statement's module.
    #[test_matrix(
        [
            "user new_user with password 'password'",
            "user mapping for foo server bar",
        ]
        => matches Ok(_)
    )]
    fn test_create_user_stmt(source: &str) -> scan::Result<RawStmt> {
        test_parser!(source, create_user_stmt)
    }

    #[test_case("mapping if not exists for test_user server test_server options (foo '42')",
        CreateUserMappingStmt::new(
            RoleSpec::Name("test_user".into()),
            "test_server",
            Some(vec![GenericOption::new("foo", "42")]),
            Presence::Ignore
        )
    )]
    #[test_case("mapping for foo server bar",
        CreateUserMappingStmt::new(
            RoleSpec::Name("foo".into()),
            "bar",
            None,
            Presence::Fail
        )
    )]
    fn test_create_user_mapping(source: &str, expected: CreateUserMappingStmt) {
        test_parser!(source, create_user_mapping, expected);
    }

    #[test]
    fn test_create_user_role() {
        test_parser!(
            source = "test_user with sysid 42",
            parser = create_user_role,
            expected = CreateRoleStmt::new(
                "test_user",
                RoleKind::User,
                vec![CreateRoleOption::SysId(42.into())]
            )
        )
    }
}

use crate::combinators::stmt::auth_ident;
use crate::combinators::stmt::create_stmt::create_role_options;
use crate::combinators::stmt::if_not_exists;
use pg_ast::CreateRoleStmt;
use pg_ast::CreateUserMappingStmt;
use pg_ast::RawStmt;
use pg_ast::RoleKind;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_generic_options_combinators::create_generic_options;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Mapping;
use pg_lexer::Keyword::Server;
use pg_lexer::Keyword::User;
use pg_lexer::Keyword::With;
use pg_parser_core::scan;
use pg_sink_combinators::col_id;
use pg_sink_combinators::role_id;
