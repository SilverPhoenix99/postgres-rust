pub type LocatedError = LocatedMessage<Error>;
pub type Result<T = Str> = core::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    /// When "none" or "public" was incorrectly used as a role.
    #[error(r#"role name "{0}" is reserved"#)]
    ReservedRoleSpec(&'static str),

    /// When a role is disallowed
    #[error(r"{0} cannot be used as a role name here")]
    ForbiddenRoleSpec(&'static str),
}

impl Error {
    pub fn at(self, location: Location) -> LocatedError {
        LocatedError::new(self, location)
    }
}

impl LogMessage for Error {

    fn sql_state(&self) -> SqlState {
        SqlState::ReservedName
    }
}

use crate::LocatedMessage;
use crate::LogMessage;
use crate::SqlState;
use pg_basics::Location;
use pg_basics::Str;
