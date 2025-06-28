mod in_database;
mod user_stmt;

/// Aliases:
/// * `AlterRoleStmt`
/// * `AlterRoleSetStmt`
/// * `AlterUserMappingStmt`
pub(super) fn alter_user_stmt() -> impl Combinator<Output = RawStmt> {

    /*
          ALTER USER MAPPING FOR auth_ident SERVER ColId alter_generic_options  => AlterUserMappingStmt
        | ALTER ( ROLE | USER ) user_stmt                                       => RawStmt
    */

    match_first! {
        User.and_right(match_first! {
            (Mapping, For, auth_ident, Server, col_id, alter_generic_options())
                .map(
                    |(_, _, user, _, servername, options)|
                        AlterUserMappingStmt::new(user, servername, options).into()
                ),
            user_stmt()
        }),
        Kw::Role.and_right(user_stmt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::AddDrop::Add;
    use pg_ast::AlterRoleStmt;
    use pg_ast::GenericOption;
    use pg_ast::GenericOptionKind::Unspecified;
    use pg_ast::RoleSpec::CurrentUser;
    use pg_ast::RoleSpec::Public;

    #[test]
    fn test_user_mapping() {
        let source = "user mapping for user server server_name options ( x 'y' )";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_user_stmt().parse(&mut stream);

        let expected = AlterUserMappingStmt::new(
            CurrentUser,
            "server_name",
            vec![Unspecified(GenericOption::new("x", "y"))]
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_role() {
        let source = "role public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_user_stmt().parse(&mut stream);

        let expected = AlterRoleStmt::new(Public, Add, None);

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_user() {
        let source = "user public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_user_stmt().parse(&mut stream);

        let expected = AlterRoleStmt::new(Public, Add, None);

        assert_eq!(Ok(expected.into()), actual);
    }
}

use self::user_stmt::user_stmt;
use crate::combinators::col_id;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::stmt::alter_stmt::alter_generic_options;
use crate::combinators::stmt::auth_ident;
use pg_ast::AlterUserMappingStmt;
use pg_ast::RawStmt;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Mapping;
use pg_lexer::Keyword::Server;
use pg_lexer::Keyword::User;
