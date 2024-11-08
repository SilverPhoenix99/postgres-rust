#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(CowStr),
}

impl RoleSpec {
    pub(in crate::parser) fn into_role_id(self) -> ParseResult<CowStr> {
        const FN_NAME: &str = "postgres_parser::parser::ast_node::RoleSpec::into_role_id";
        match self {
            Self::Name(role) => Ok(role),
            Self::Public => Err(ReservedRoleSpec("public").with_fn_info(fn_info!(FN_NAME))),
            Self::CurrentRole => Err(ForbiddenRoleSpec("CURRENT_ROLE").with_fn_info( fn_info!(FN_NAME))),
            Self::CurrentUser => Err(ForbiddenRoleSpec("CURRENT_USER").with_fn_info( fn_info!(FN_NAME))),
            Self::SessionUser => Err(ForbiddenRoleSpec("SESSION_USER").with_fn_info( fn_info!(FN_NAME))),
        }
    }
}

use crate::parser::{
    ast_node::CowStr,
    ParseResult,
    ParserErrorKind::{ForbiddenRoleSpec, ReservedRoleSpec}
};
use postgres_basics::fn_info;
