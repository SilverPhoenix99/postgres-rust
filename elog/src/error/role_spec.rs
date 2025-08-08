pub type LocatedError = LocatedMessage<Error>;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Display, Error)]
pub enum Error {

    #[display(r#"role name "{role}" is reserved"#)]
    ReservedRoleSpec {
        #[error(not(source))]
        role: & 'static str
    },

    #[display(r"{role} cannot be used as a role name here")]
    ForbiddenRoleSpec {
        #[error(not(source))]
        role: & 'static str
    },
}

impl Error {
    pub fn at(self, location: Location) -> LocatedError {
        LocatedError::new(self, location)
    }
}

impl LogMessage for Error {
    fn sql_state(&self) -> SqlState {
        ReservedName
    }
}

use crate::LocatedMessage;
use crate::LogMessage;
use crate::SqlState;
use crate::SqlState::ReservedName;
use derive_more::Display;
use derive_more::Error;
use pg_basics::Location;
