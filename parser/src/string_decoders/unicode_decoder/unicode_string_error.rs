#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum UnicodeStringError {

    /// Invalid UTF-8 char.
    // FIXME: Doesn't seem to exist in `parser.c` -> `str_udeescape()`
    #[error(r#"invalid byte sequence for encoding "UTF8""#)]
    Utf8(Utf8Error),

    /// When the `\XXXX`|`\+XXXXXX` escape is invalid UTF-32.
    #[error("invalid Unicode escape value")]
    InvalidUnicodeValue(usize),

    /// When
    #[error("invalid Unicode surrogate pair")]
    InvalidUnicodeSurrogatePair(usize),

    /// When the format of the escape doesn't match \uXXXX or \UXXXXXXXX.
    #[error("invalid Unicode escape")]
    InvalidUnicodeEscape(usize),
}

impl UnicodeStringError {

    pub fn sqlstate(self) -> SqlState {
        match self {
            Utf8(_) => Error(CharacterNotInRepertoire),
            InvalidUnicodeValue(_) => Error(SyntaxError),
            InvalidUnicodeSurrogatePair(_) => Error(SyntaxError),
            InvalidUnicodeEscape(_) => Error(SyntaxError),
        }
    }

    pub fn hint(self) -> Option<&'static str> {
        match self {
            Utf8(_) => None,
            InvalidUnicodeValue(_) => None,
            InvalidUnicodeSurrogatePair(_) => None,
            InvalidUnicodeEscape(_) => Some(r"Unicode escapes must be \XXXX or \+XXXXXX."),
        }
    }
}

use postgres_basics::sql_state::{
    ErrorSqlState::{CharacterNotInRepertoire, SyntaxError},
    SqlState,
    SqlState::Error
};
use std::str::Utf8Error;
use UnicodeStringError::*;
