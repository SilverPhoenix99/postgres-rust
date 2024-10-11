impl Parser<'_> {
    /// Aliases:
    /// * `RevokeStmt`
    /// * `RevokeRoleStmt`
    pub(in crate::parser) fn revoke_stmt(&mut self) -> ScanResult<RawStmt> {

        /*
            REVOKE privileges ON privilege_target FROM grantee_list opt_granted_by opt_drop_behavior
            REVOKE GRANT OPTION FOR privileges ON privilege_target FROM grantee_list opt_granted_by opt_drop_behavior
            REVOKE privilege_list FROM role_list opt_granted_by opt_drop_behavior
            REVOKE ColId OPTION FOR privilege_list FROM role_list opt_granted_by opt_drop_behavior
        */

        self.buffer.consume_kw_eq(Revoke)?;

        todo!()
    }
}

use crate::lexer::Keyword::Revoke;
use crate::parser::ast_node::RawStmt;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
