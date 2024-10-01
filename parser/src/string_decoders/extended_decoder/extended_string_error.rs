#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum ExtendedStringError {

    /// Invalid UTF-8 char.
    // TODO: yyerror, aka scanner_yyerror (i.e., "%s at end of input" vs r#"%s at or near "%s""#)
    #[error(r#"invalid byte sequence for encoding "UTF8""#)]
    Utf8(Utf8Error),

    /// When the result of parsing the `\uXXXX`|`\UXXXXXXXX` escape gives back invalid UTF-16/UTF-32.
    #[error("invalid Unicode escape value")]
    InvalidUnicodeValue(usize),

    /// Invalid UTF-16 surrogate pair.
    // TODO: yyerror, aka scanner_yyerror
    #[error("invalid Unicode surrogate pair")]
    InvalidUnicodeSurrogatePair(usize),

    /// When the string uses the unsafe `\'` escape
    #[error(r"unsafe use of \' in a string literal")]
    NonstandardUseOfBackslashQuote,

    /// When the format of the escape doesn't match \uXXXX or \UXXXXXXXX.
    #[error("invalid Unicode escape")]
    InvalidUnicodeEscape(usize),
}

impl HasSqlState for ExtendedStringError {
    fn sql_state(&self) -> SqlState {
        match self {
            Utf8(_) => Error(CharacterNotInRepertoire),
            InvalidUnicodeValue(_) => Error(SyntaxError),
            InvalidUnicodeSurrogatePair(_) => Error(SyntaxError),
            NonstandardUseOfBackslashQuote => Error(NonstandardUseOfEscapeCharacter),
            InvalidUnicodeEscape(_) => Error(InvalidEscapeSequence),
        }
    }
}

impl ErrorReport for ExtendedStringError {
    fn hint(&self) -> Option<Cow<'static, str>> {
        match self {
            NonstandardUseOfBackslashQuote => Some(
                r"Use '' to write quotes in strings. \' is insecure in client-only encodings.".into()
            ),
            InvalidUnicodeEscape(_) => Some(r"Unicode escapes must be \uXXXX or \UXXXXXXXX.".into()),
            _ => None,
        }
    }
}

use postgres_basics::{
    elog::{ErrorReport, HasSqlState},
    sql_state::ErrorSqlState::{
        CharacterNotInRepertoire,
        InvalidEscapeSequence,
        NonstandardUseOfEscapeCharacter,
        SyntaxError
    },
    sql_state::SqlState,
    sql_state::SqlState::Error
};
use std::borrow::Cow;
use std::str::Utf8Error;
use ExtendedStringError::*;
