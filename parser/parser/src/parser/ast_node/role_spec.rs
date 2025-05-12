#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(Str),
}

impl RoleSpec {
    pub(in crate::parser) fn into_role_id(self, location: Location) -> ScanResult<Str> {

        let err = match self {
            Self::Name(role) => return Ok(role),
            Self::Public => ReservedRoleSpec("public"),
            Self::CurrentRole => ForbiddenRoleSpec("CURRENT_ROLE"),
            Self::CurrentUser => ForbiddenRoleSpec("CURRENT_USER"),
            Self::SessionUser => ForbiddenRoleSpec("SESSION_USER"),
        };

        Err(ParserError::new(err, location).into())
    }
}

use crate::parser::result::ScanResult;
use crate::parser::ParserError;
use crate::parser::ParserErrorKind::ForbiddenRoleSpec;
use crate::parser::ParserErrorKind::ReservedRoleSpec;
use postgres_basics::Location;
use postgres_basics::Str;
