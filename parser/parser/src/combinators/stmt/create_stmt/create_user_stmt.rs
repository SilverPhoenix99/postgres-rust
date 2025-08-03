pub(super) fn create_user_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

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
    ).parse(stream)?;

    Ok(stmt)
}

fn create_user_mapping(stream: &mut TokenStream) -> scan::Result<CreateUserMappingStmt> {

    let (_, existence, _, user, _, server, options) = seq!(
        Mapping,
        if_not_exists.optional()
            .map(Option::unwrap_or_default),
        For,
        auth_ident,
        Server,
        col_id,
        create_generic_options.optional()
    ).parse(stream)?;

    let stmt = CreateUserMappingStmt::new(user, server, options, existence);
    Ok(stmt)
}

fn create_user_role(stream: &mut TokenStream) -> scan::Result<CreateRoleStmt> {

    let (name, _, options) = seq!(
        role_id,
        With.optional(),
        create_role_options
    ).parse(stream)?;

    let stmt = CreateRoleStmt::new(name, RoleKind::User, options);
    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::test_parser;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::CreateRoleOption;
    #[allow(unused_imports)]
    use pg_ast::{
        GenericOption,
        Presence,
        RoleSpec
    };
    use test_case::test_case;

    #[test_case("user new_user with password 'password'")]
    #[test_case("user mapping for foo server bar")]
    fn test_create_user_stmt(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = create_user_stmt(&mut stream);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
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

use crate::combinators::col_id;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_id;
use crate::combinators::stmt::auth_ident;
use crate::combinators::stmt::create_generic_options;
use crate::combinators::stmt::create_stmt::create_role_options;
use crate::combinators::stmt::if_not_exists;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::CreateRoleStmt;
use pg_ast::CreateUserMappingStmt;
use pg_ast::RawStmt;
use pg_ast::RoleKind;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Mapping;
use pg_lexer::Keyword::Server;
use pg_lexer::Keyword::User;
use pg_lexer::Keyword::With;
