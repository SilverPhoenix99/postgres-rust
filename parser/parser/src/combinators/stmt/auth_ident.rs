pub(super) fn auth_ident(stream: &mut TokenStream) -> scan::Result<RoleSpec> {

    choice!(parsed stream =>
        User.map(|_| CurrentUser),
        role_spec
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("public", RoleSpec::Public)]
    #[test_case("user", CurrentUser)]
    fn test_auth_ident(source: &str, expected: RoleSpec) {
        test_parser!(source, auth_ident, expected)
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_spec;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RoleSpec;
use pg_ast::RoleSpec::CurrentUser;
use pg_lexer::Keyword::User;
