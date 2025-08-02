pub(super) fn role(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
          CURRENT_ROLE
        | CURRENT_USER
        | SESSION_USER
        | SYSTEM_USER
        | USER
    */

    or((
        Kw::CurrentRole.map(|_| CurrentRole),
        Kw::CurrentUser.map(|_| CurrentUser),
        Kw::SessionUser.map(|_| SessionUser),
        Kw::SystemUser.map(|_| SystemUser),
        Kw::User.map(|_| User),
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("CURRENT_role" => Ok(CurrentRole))]
    #[test_case("current_USER" => Ok(CurrentUser))]
    #[test_case("SESSION_USER" => Ok(SessionUser))]
    #[test_case("system_user" => Ok(SystemUser))]
    #[test_case("uSeR" => Ok(User))]
    fn test_role(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, role)
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_ast::ExprNode::CurrentRole;
use pg_ast::ExprNode::CurrentUser;
use pg_ast::ExprNode::SessionUser;
use pg_ast::ExprNode::SystemUser;
use pg_ast::ExprNode::User;
use pg_lexer::Keyword as Kw;
