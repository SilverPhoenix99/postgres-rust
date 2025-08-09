pub type LocatedError = Located<Error>;
pub type Result<T = Box<str>> = core::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Display)]
pub enum Error {

    /// When the result of parsing the `\XXXX`|`\+XXXXXX` escape gives back invalid UTF-16/UTF-32.
    #[display("invalid Unicode escape value")]
    InvalidUnicodeValue(u32),

    /// When
    #[display("invalid Unicode surrogate pair")]
    InvalidUnicodeSurrogatePair(u32),

    /// When the format of the escape doesn't match \XXXX or \+XXXXXX.
    #[display("invalid Unicode escape")]
    InvalidUnicodeEscape(u32),
}

impl core::error::Error for Error {}

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
use crate::LogMessage;
use derive_more::Display;
use pg_basics::Located;
