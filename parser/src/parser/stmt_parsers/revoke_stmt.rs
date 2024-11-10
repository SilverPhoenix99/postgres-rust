impl Parser<'_> {
    /// Aliases:
    /// * `RevokeStmt`
    /// * `RevokeRoleStmt`
    pub(in crate::parser) fn revoke_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            REVOKE privileges ON privilege_target FROM grantee_list opt_granted_by opt_drop_behavior
            REVOKE GRANT OPTION FOR privileges ON privilege_target FROM grantee_list opt_granted_by opt_drop_behavior
            REVOKE privilege_list FROM role_list opt_granted_by opt_drop_behavior
            REVOKE ColId OPTION FOR privilege_list FROM role_list opt_granted_by opt_drop_behavior
        */

        todo!()
    }
}

use crate::parser::{
    ast_node::RawStmt,
    ParseResult,
    Parser
};
