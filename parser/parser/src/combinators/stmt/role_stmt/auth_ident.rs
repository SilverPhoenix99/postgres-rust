pub(super) fn auth_ident(ctx: &mut ParserContext) -> scan::Result<RoleSpec> {

    alt!(
        User.map(|_| CurrentUser),
        role_spec
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("public" => Ok(RoleSpec::Public))]
    #[test_matrix("user" => Ok(CurrentUser))]
    fn test_auth_ident(source: &str) -> scan::Result<RoleSpec> {
        test_parser!(source, auth_ident)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::role_spec;
use crate::ParserContext;
use pg_ast::RoleSpec;
use pg_ast::RoleSpec::CurrentUser;
use pg_lexer::Keyword::User;
use pg_parser_core::scan;
