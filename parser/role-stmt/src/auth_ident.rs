pub fn auth_ident(ctx: &mut ParserContext) -> scan::Result<RoleSpec> {

    alt!(
        User.map(|_| CurrentUser),
        role_spec
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("public", RoleSpec::Public)]
    #[test_case("user", CurrentUser)]
    fn test_auth_ident(source: &str, expected: RoleSpec) {
        test_parser!(source, auth_ident, expected)
    }
}

use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::User;
use pg_parser_core::scan;
use pg_sink_ast::RoleSpec;
use pg_sink_ast::RoleSpec::CurrentUser;
use pg_sink_combinators::role_spec;
