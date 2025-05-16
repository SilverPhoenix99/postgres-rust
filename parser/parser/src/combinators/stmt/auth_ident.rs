pub(super) fn auth_ident() -> impl Combinator<Output = RoleSpec> {

    or(
        User.map(|_| CurrentUser),
        role_spec()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("public", RoleSpec::Public)]
    #[test_case("user", CurrentUser)]
    fn test_auth_ident(source: &str, expected: RoleSpec) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = auth_ident().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::role_spec;
use postgres_parser_ast::RoleSpec;
use postgres_parser_ast::RoleSpec::CurrentUser;
use postgres_parser_lexer::Keyword::User;
