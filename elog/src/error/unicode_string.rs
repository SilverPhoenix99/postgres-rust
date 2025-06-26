pub type LocatedError = LocatedMessage<Error>;
pub type Result<T = Box<str>> = core::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Error {

    /// When the result of parsing the `\XXXX`|`\+XXXXXX` escape gives back invalid UTF-16/UTF-32.
    #[error("invalid Unicode escape value")]
    InvalidUnicodeValue(u32),

    /// When
    #[error("invalid Unicode surrogate pair")]
    InvalidUnicodeSurrogatePair(u32),

    /// When the format of the escape doesn't match \XXXX or \+XXXXXX.
    #[error("invalid Unicode escape")]
    InvalidUnicodeEscape(u32),
}

impl Error {
    pub fn at(self, location: Location) -> LocatedError {
        LocatedError::new(self, location)
    }
}

impl LogMessage for Error {

    fn sql_state(&self) -> SqlState {
        SyntaxError
    }

    fn hint(&self) -> Option<&str> {
        match self {
            Self::InvalidUnicodeEscape(_) => Some(r"Unicode escapes must be \XXXX or \+XXXXXX."),
            _ => None,
        }
    }
}

use crate::sql_state::SqlState;
use crate::sql_state::SqlState::SyntaxError;
use crate::LocatedMessage;
use crate::LogMessage;
use pg_basics::Location;
