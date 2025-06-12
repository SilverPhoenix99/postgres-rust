#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum ExtendedStringError {

    /// Invalid UTF-8 char.
    // TODO: yyerror, aka scanner_yyerror (i.e., "%s at end of input" vs r#"%s at or near "%s""#)
    #[error(r#"invalid byte sequence for encoding "UTF8""#)]
    Utf8(Utf8Error),

    /// When the result of parsing the `\uXXXX`|`\UXXXXXXXX` escape gives back invalid UTF-16/UTF-32.
    #[error("invalid Unicode escape value")]
    InvalidUnicodeValue(u32),

    /// Invalid UTF-16 surrogate pair.
    // TODO: yyerror, aka scanner_yyerror
    #[error("invalid Unicode surrogate pair")]
    InvalidUnicodeSurrogatePair(u32),

    /// When the string uses the unsafe `\'` escape
    #[error(r"unsafe use of \' in a string literal")]
    NonstandardUseOfBackslashQuote,

    /// When the format of the escape doesn't match \uXXXX or \UXXXXXXXX.
    #[error("invalid Unicode escape")]
    InvalidUnicodeEscape(u32),
}

impl Error for ExtendedStringError {

    fn sql_state(&self) -> SqlState {
        match self {
            Utf8(_) => CharacterNotInRepertoire,
            InvalidUnicodeValue(_) => SyntaxError,
            InvalidUnicodeSurrogatePair(_) => SyntaxError,
            NonstandardUseOfBackslashQuote => NonstandardUseOfEscapeCharacter,
            InvalidUnicodeEscape(_) => InvalidEscapeSequence,
        }
    }

    fn hint(&self) -> Option<Str> {
        match self {
            NonstandardUseOfBackslashQuote => Some(
                r"Use '' to write quotes in strings. \' is insecure in client-only encodings.".into()
            ),
            InvalidUnicodeEscape(_) => Some(r"Unicode escapes must be \uXXXX or \UXXXXXXXX.".into()),
            _ => None,
        }
    }
}

use crate::sql_state::SqlState;
use crate::sql_state::SqlState::CharacterNotInRepertoire;
use crate::sql_state::SqlState::InvalidEscapeSequence;
use crate::sql_state::SqlState::NonstandardUseOfEscapeCharacter;
use crate::sql_state::SqlState::SyntaxError;
use crate::Error;
use pg_basics::Str;
use std::str::Utf8Error;
use ExtendedStringError::*;
