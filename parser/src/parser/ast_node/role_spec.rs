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
            Self::Public => Err(PartialParserError::new(ReservedRoleSpec("public"), fn_info!(FN_NAME))),
            Self::CurrentRole => Err(PartialParserError::new(ForbiddenRoleSpec("CURRENT_ROLE"), fn_info!(FN_NAME))),
            Self::CurrentUser => Err(PartialParserError::new(ForbiddenRoleSpec("CURRENT_USER"), fn_info!(FN_NAME))),
            Self::SessionUser => Err(PartialParserError::new(ForbiddenRoleSpec("SESSION_USER"), fn_info!(FN_NAME))),
        }
    }
}

use crate::parser::{
    ast_node::CowStr,
    error::PartialParserError,
    ParseResult,
    ParserErrorKind::{ForbiddenRoleSpec, ReservedRoleSpec}
};
use postgres_basics::fn_info;
