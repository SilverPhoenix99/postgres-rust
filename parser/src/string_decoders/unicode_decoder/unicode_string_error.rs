#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum UnicodeStringError {

    /// Invalid UTF-8 char.
    // FIXME: Doesn't seem to exist in `parser.c` -> `str_udeescape()`
    #[error(r#"invalid byte sequence for encoding "UTF8""#)]
    Utf8(Utf8Error),

    /// When the result of parsing the `\XXXX`|`\+XXXXXX` escape gives back invalid UTF-16/UTF-32.
    #[error("invalid Unicode escape value")]
    InvalidUnicodeValue(usize),

    /// When
    #[error("invalid Unicode surrogate pair")]
    InvalidUnicodeSurrogatePair(usize),

    /// When the format of the escape doesn't match \XXXX or \+XXXXXX.
    #[error("invalid Unicode escape")]
    InvalidUnicodeEscape(usize), 
}

impl HasSqlState for UnicodeStringError {
    fn sql_state(&self) -> SqlState {
        match self {
            Utf8(_) => Error(CharacterNotInRepertoire),
            InvalidUnicodeValue(_) => Error(SyntaxError),
            InvalidUnicodeSurrogatePair(_) => Error(SyntaxError),
            InvalidUnicodeEscape(_) => Error(SyntaxError),
        }
    }
}

impl ErrorReport for UnicodeStringError {
    fn hint(&self) -> Option<Cow<'static, str>> {
        match self {
            InvalidUnicodeEscape(_) => Some(r"Unicode escapes must be \XXXX or \+XXXXXX.".into()),
            _ => None,
        }
    }
}

use postgres_basics::{
    elog::{ErrorReport, HasSqlState},
    sql_state::ErrorSqlState::{CharacterNotInRepertoire, SyntaxError},
    sql_state::SqlState,
    sql_state::SqlState::Error,
};
use std::borrow::Cow;
use std::str::Utf8Error;
use UnicodeStringError::*;
