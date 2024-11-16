#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(Str),
}

impl RoleSpec {
    pub(in crate::parser) fn into_role_id(self, location: Location) -> ParseResult<Str> {

        let err = match self {
            Self::Name(role) => return Ok(role),
            Self::Public => ReservedRoleSpec("public"),
            Self::CurrentRole => ForbiddenRoleSpec("CURRENT_ROLE"),
            Self::CurrentUser => ForbiddenRoleSpec("CURRENT_USER"),
            Self::SessionUser => ForbiddenRoleSpec("SESSION_USER"),
        };

        Err(ParserError::new(err, fn_info!(), location))
    }
}

use crate::parser::{
    ParseResult,
    ParserError,
    ParserErrorKind::{ForbiddenRoleSpec, ReservedRoleSpec}
};
use postgres_basics::{fn_info, Location, Str};
