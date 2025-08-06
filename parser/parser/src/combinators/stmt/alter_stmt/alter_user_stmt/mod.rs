mod in_database;
mod user_stmt;

/// Aliases:
/// * `AlterRoleStmt`
/// * `AlterRoleSetStmt`
/// * `AlterUserMappingStmt`
pub(super) fn alter_user_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
          ALTER USER MAPPING FOR auth_ident SERVER ColId alter_generic_options  => AlterUserMappingStmt
        | ALTER ( ROLE | USER ) user_stmt                                       => RawStmt
    */

    alt!(
        seq!(
            User,
            alt!(
                seq!(Mapping, For, auth_ident, Server, col_id, alter_generic_options)
                    .map(|(_, _, user, _, servername, options)|
                        AlterUserMappingStmt::new(user, servername, options).into()
                    ),
                user_stmt
            )
        )
            .map(|(_, stmt)| stmt),
        seq!(Kw::Role, user_stmt)
            .map(|(_, stmt)| stmt)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::AddDrop::Add;
    use pg_ast::AlterRoleStmt;
    use pg_ast::GenericOption;
    use pg_ast::GenericOptionKind::Unspecified;
    use pg_ast::RoleSpec::CurrentUser;
    use pg_ast::RoleSpec::Public;
    use pg_combinators::test_parser;

    #[test]
    fn test_user_mapping() {
        test_parser!(
            source = "user mapping for user server server_name options ( x 'y' )",
            parser = alter_user_stmt,
            expected = AlterUserMappingStmt::new(
                CurrentUser,
                "server_name",
                vec![Unspecified(GenericOption::new("x", "y"))]
            )
        )
    }

    #[test]
    fn test_role() {
        test_parser!(
            source = "role public",
            parser = alter_user_stmt,
            expected = AlterRoleStmt::new(Public, Add, None)
        )
    }

    #[test]
    fn test_user() {
        test_parser!(
            source = "user public",
            parser = alter_user_stmt,
            expected = AlterRoleStmt::new(Public, Add, None)
        )
    }
}

use self::user_stmt::user_stmt;
use crate::combinators::stmt::alter_stmt::alter_generic_options;
use crate::combinators::stmt::auth_ident;
use pg_ast::AlterUserMappingStmt;
use pg_ast::RawStmt;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_identifier_combinators::col_id;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Mapping;
use pg_lexer::Keyword::Server;
use pg_lexer::Keyword::User;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
