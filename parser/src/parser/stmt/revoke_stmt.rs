/// Aliases:
/// * `RevokeStmt`
/// * `RevokeRoleStmt`
pub(super) fn revoke_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        REVOKE privileges ON privilege_target FROM grantee_list opt_granted_by opt_drop_behavior
        REVOKE GRANT OPTION FOR privileges ON privilege_target FROM grantee_list opt_granted_by opt_drop_behavior
        REVOKE privilege_list FROM role_list opt_granted_by opt_drop_behavior
        REVOKE ColId OPTION FOR privilege_list FROM role_list opt_granted_by opt_drop_behavior
    */

    Revoke
        .map(|_| todo!())
}

use crate::lexer::Keyword::Revoke;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
