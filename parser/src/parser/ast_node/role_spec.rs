#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(CowStr),
}

impl RoleSpec {
    pub fn into_role_id(self) -> ParseResult<CowStr> {
        match self {
            Self::Name(role) => Ok(role),
            Self::Public => Err(ReservedRoleSpec("public")),
            Self::CurrentRole => Err(ForbiddenRoleSpec("CURRENT_ROLE")),
            Self::CurrentUser => Err(ForbiddenRoleSpec("CURRENT_USER")),
            Self::SessionUser => Err(ForbiddenRoleSpec("SESSION_USER")),
        }
    }
}

use crate::parser::ast_node::CowStr;
use crate::parser::ParserErrorKind::{ForbiddenRoleSpec, ReservedRoleSpec};
use crate::parser::ParseResult;
