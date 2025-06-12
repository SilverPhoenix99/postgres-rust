#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum UnicodeStringError {

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

impl Error for UnicodeStringError {

    fn sql_state(&self) -> SqlState {
        SyntaxError
    }
    
    fn hint(&self) -> Option<Str> {
        match self {
            InvalidUnicodeEscape(_) => Some(r"Unicode escapes must be \XXXX or \+XXXXXX.".into()),
            _ => None,
        }
    }
}

use crate::sql_state::SqlState;
use crate::sql_state::SqlState::SyntaxError;
use crate::Error;
use pg_basics::Str;
use UnicodeStringError::*;
