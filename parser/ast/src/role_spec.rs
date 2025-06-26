#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(Str),
}

impl RoleSpec {
    pub fn into_role_id(self) -> role_spec::Result {

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

use pg_basics::Str;
use pg_elog::role_spec;
use pg_elog::role_spec::Error::ForbiddenRoleSpec;
use pg_elog::role_spec::Error::ReservedRoleSpec;
