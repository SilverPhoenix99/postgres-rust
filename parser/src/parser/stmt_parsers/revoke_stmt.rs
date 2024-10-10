impl Parser<'_> {
    /// Aliases:
    /// * `RevokeStmt`
    /// * `RevokeRoleStmt`
    pub(in crate::parser) fn revoke_stmt(&mut self) -> Result<AstNode, ScanErrorKind> {

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
use crate::parser::ast_node::AstNode;
use crate::parser::result::ScanErrorKind;
use crate::parser::Parser;
