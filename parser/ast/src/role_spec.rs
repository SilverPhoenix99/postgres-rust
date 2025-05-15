#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(Str),
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

use elog::RoleSpecError;
use elog::RoleSpecError::ForbiddenRoleSpec;
use elog::RoleSpecError::ReservedRoleSpec;
use postgres_basics::Str;
