pub type LocatedError = Located<Error>;
pub type Result<T = Box<str>> = core::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Display)]
pub enum Error {

    /// Invalid UTF-8 char.
    // TODO: yyerror, aka scanner_yyerror (i.e., "%s at end of input" vs r#"%s at or near "%s""#)
    #[display(r#"invalid byte sequence for encoding "UTF8""#)]
    Utf8(Utf8Error),

    /// When the result of parsing the `\uXXXX`|`\UXXXXXXXX` escape gives back invalid UTF-16/UTF-32.
    #[display("invalid Unicode escape value")]
    InvalidUnicodeValue(u32),

    /// Invalid UTF-16 surrogate pair.
    // TODO: yyerror, aka scanner_yyerror
    #[display("invalid Unicode surrogate pair")]
    InvalidUnicodeSurrogatePair(u32),

    /// When the string uses the unsafe `\'` escape
    #[display(r"unsafe use of \' in a string literal")]
    NonstandardUseOfBackslashQuote,

    /// When the format of the escape doesn't match \uXXXX or \UXXXXXXXX.
    #[display("invalid Unicode escape")]
    InvalidUnicodeEscape(u32),
}

impl core::error::Error for Error {}

impl LogMessage for Error {

    fn sql_state(&self) -> SqlState {
        match self {
            Self::Utf8(_) => CharacterNotInRepertoire,
            Self::InvalidUnicodeValue(_) => SyntaxError,
            Self::InvalidUnicodeSurrogatePair(_) => SyntaxError,
            Self::NonstandardUseOfBackslashQuote => NonstandardUseOfEscapeCharacter,
            Self::InvalidUnicodeEscape(_) => InvalidEscapeSequence,
        }
    }

    fn hint(&self) -> Option<&str> {
        match self {
            Self::NonstandardUseOfBackslashQuote => Some(
                r"Use '' to write quotes in strings. \' is insecure in client-only encodings."
            ),
            Self::InvalidUnicodeEscape(_) => Some(r"Unicode escapes must be \uXXXX or \UXXXXXXXX."),
            _ => None,
        }
    }
}

use crate::sql_state::SqlState;
use crate::sql_state::SqlState::CharacterNotInRepertoire;
use crate::sql_state::SqlState::InvalidEscapeSequence;
use crate::sql_state::SqlState::NonstandardUseOfEscapeCharacter;
use crate::sql_state::SqlState::SyntaxError;
use crate::LogMessage;
use core::str::Utf8Error;
use derive_more::Display;
use pg_basics::Located;
