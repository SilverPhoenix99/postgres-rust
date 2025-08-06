/// Aliases:
/// * `RevokeStmt`
/// * `RevokeRoleStmt`
pub(super) fn revoke_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
          REVOKE privileges ON privilege_target FROM grantee_list opt_granted_by opt_drop_behavior
        | REVOKE GRANT OPTION FOR privileges ON privilege_target FROM grantee_list opt_granted_by opt_drop_behavior
        | REVOKE privilege_list FROM role_list opt_granted_by opt_drop_behavior
        | REVOKE ColId OPTION FOR privilege_list FROM role_list opt_granted_by opt_drop_behavior
    */

    let (_, stmt) = seq!(Revoke, parser(|_| todo!()))
        .parse(stream)?;

    Ok(stmt)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Revoke;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
