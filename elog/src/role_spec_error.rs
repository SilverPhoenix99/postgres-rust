pub type RoleSpecError = LocatedError<RoleSpecErrorKind>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum RoleSpecErrorKind {
    /// When "none" or "public" was incorrectly used as a role.
    #[error(r#"role name "{0}" is reserved"#)]
    ReservedRoleSpec(&'static str),

    /// When a role is disallowed
    #[error(r"{0} cannot be used as a role name here")]
    ForbiddenRoleSpec(&'static str),
}

impl Error for RoleSpecErrorKind {

    fn sql_state(&self) -> SqlState {
        ReservedName
    }
}

use crate::sql_state::SqlState;
use crate::sql_state::SqlState::ReservedName;
use crate::Error;
use crate::LocatedError;
