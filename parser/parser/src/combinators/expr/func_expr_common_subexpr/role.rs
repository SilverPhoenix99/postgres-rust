pub(super) fn role(stream: &mut TokenStream) -> scan::Result<SqlFunction> {

    /*
          CURRENT_ROLE
        | CURRENT_USER
        | SESSION_USER
        | SYSTEM_USER
        | USER
    */

    alt!(
        Kw::CurrentRole.map(|_| CurrentRole),
        Kw::CurrentUser.map(|_| CurrentUser),
        Kw::SessionUser.map(|_| SessionUser),
        Kw::SystemUser.map(|_| SystemUser),
        Kw::User.map(|_| User),
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("CURRENT_role" => Ok(CurrentRole))]
    #[test_case("current_USER" => Ok(CurrentUser))]
    #[test_case("SESSION_USER" => Ok(SessionUser))]
    #[test_case("system_user" => Ok(SystemUser))]
    #[test_case("uSeR" => Ok(User))]
    fn test_role(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, role)
    }
}

use pg_ast::SqlFunction;
use pg_ast::SqlFunction::CurrentRole;
use pg_ast::SqlFunction::CurrentUser;
use pg_ast::SqlFunction::SessionUser;
use pg_ast::SqlFunction::SystemUser;
use pg_ast::SqlFunction::User;
use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
