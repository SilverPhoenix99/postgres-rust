#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(CowStr),
}

impl RoleSpec {
    pub(in crate::parser) fn into_role_id(self, location: Location) -> ParseResult<CowStr> {
        const FN_NAME: &str = "postgres_parser::parser::ast_node::RoleSpec::into_role_id";

        let err = match self {
            Self::Name(role) => return Ok(role),
            Self::Public => ReservedRoleSpec("public"),
            Self::CurrentRole => ForbiddenRoleSpec("CURRENT_ROLE"),
            Self::CurrentUser => ForbiddenRoleSpec("CURRENT_USER"),
            Self::SessionUser => ForbiddenRoleSpec("SESSION_USER"),
        };

        Err(ParserError::new(err, fn_info!(FN_NAME), location))
    }
}

use crate::parser::{
    ast_node::CowStr,
    ParseResult,
    ParserError,
    ParserErrorKind::{ForbiddenRoleSpec, ReservedRoleSpec}
};
use postgres_basics::{fn_info, Location};
