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
            sequence!(Mapping, For, auth_ident(), Server, col_id(), alter_generic_options()).map(
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
    use crate::parser::ast_node::AddDrop::Add;
    use crate::parser::ast_node::AlterRoleStmt;
    use crate::parser::ast_node::GenericOption;
    use crate::parser::ast_node::GenericOptionKind::Unspecified;
    use crate::parser::ast_node::RoleSpec::CurrentUser;
    use crate::parser::ast_node::RoleSpec::Public;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

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

        let expected = AlterRoleStmt::new(Public, Add, vec![]);

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_user() {
        let source = "user public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_user_stmt().parse(&mut stream);

        let expected = AlterRoleStmt::new(Public, Add, vec![]);

        assert_eq!(Ok(expected.into()), actual);
    }
}

use self::user_stmt::user_stmt;
use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::For;
use crate::lexer::Keyword::Mapping;
use crate::lexer::Keyword::Server;
use crate::lexer::Keyword::User;
use crate::parser::ast_node::AlterUserMappingStmt;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::stmt::alter_stmt::alter_generic_options;
use crate::parser::combinators::stmt::auth_ident;
