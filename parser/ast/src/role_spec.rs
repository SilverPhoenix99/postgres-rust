#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(Str),
}

impl RoleSpec {
    pub fn into_role_id(self) -> Result {

        let err = match self {
            Self::Name(role) => return Ok(role),
            Self::Public => Error::ReservedRoleSpec("public"),
            Self::CurrentRole => Error::ForbiddenRoleSpec("CURRENT_ROLE"),
            Self::CurrentUser => Error::ForbiddenRoleSpec("CURRENT_USER"),
            Self::SessionUser => Error::ForbiddenRoleSpec("SESSION_USER"),
        };

        Err(err)
    }
}

use pg_basics::Str;
use pg_elog::role_spec::Error;
use pg_elog::role_spec::Result;
