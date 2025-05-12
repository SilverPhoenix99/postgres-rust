#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(Str),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RoleSpecError {
    ReservedRoleSpec(&'static str),
    ForbiddenRoleSpec(&'static str),
}

impl RoleSpec {
    pub fn into_role_id(self) -> Result<Str, RoleSpecError> {

        let err = match self {
            Self::Name(role) => return Ok(role),
            Self::Public => ReservedRoleSpec("public"),
            Self::CurrentRole => ForbiddenRoleSpec("CURRENT_ROLE"),
            Self::CurrentUser => ForbiddenRoleSpec("CURRENT_USER"),
            Self::SessionUser => ForbiddenRoleSpec("SESSION_USER"),
        };

        Err(err)
    }
}

use postgres_basics::Str;
use RoleSpecError::ForbiddenRoleSpec;
use RoleSpecError::ReservedRoleSpec;
