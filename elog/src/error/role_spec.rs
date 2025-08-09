pub type LocatedError = Located<Error>;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Display)]
pub enum Error {

    #[display(r#"role name "{role}" is reserved"#)]
    ReservedRoleSpec {
        role: & 'static str
    },

    #[display(r"{role} cannot be used as a role name here")]
    ForbiddenRoleSpec {
        role: & 'static str
    },
}

impl core::error::Error for Error {}

impl LogMessage for Error {
    fn sql_state(&self) -> SqlState {
        ReservedName
    }
}

use crate::LogMessage;
use crate::SqlState;
use crate::SqlState::ReservedName;
use derive_more::Display;
use pg_basics::Located;
