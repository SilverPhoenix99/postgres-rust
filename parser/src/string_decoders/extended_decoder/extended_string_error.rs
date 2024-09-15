use postgres_basics::sql_state::{ErrorSqlState, SqlState, SqlState::Error};
use std::str::Utf8Error;
use ExtendedStringError::*;

#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
pub enum ExtendedStringError {

    // TODO: yyerror, aka scanner_yyerror
    #[error("invalid Unicode escape value")]
    Utf8(Utf8Error),

    #[error(r"unsafe use of \' in a string literal")]
    NonstandardUseOfBackslashQuote,

    #[error("invalid Unicode escape")]
    InvalidUnicodeEscape,

    // TODO: yyerror, aka scanner_yyerror
    #[error("invalid Unicode surrogate pair")]
    InvalidUnicodeSurrogatePair,
}

impl ExtendedStringError {
    pub fn sqlstate(self) -> SqlState {
        match self {
            Utf8(_) => Error(ErrorSqlState::SyntaxError),
            NonstandardUseOfBackslashQuote => Error(ErrorSqlState::NonstandardUseOfEscapeCharacter),
            InvalidUnicodeEscape => Error(ErrorSqlState::InvalidEscapeSequence),
            InvalidUnicodeSurrogatePair => Error(ErrorSqlState::SyntaxError),
        }
    }

    pub fn hint(self) -> Option<&'static str> {
        match self {
            Utf8(_) => None,
            NonstandardUseOfBackslashQuote => Some(
                r"Use '' to write quotes in strings. \' is insecure in client-only encodings."
            ),
            InvalidUnicodeEscape => Some(r"Unicode escapes must be \uXXXX or \UXXXXXXXX."),
            InvalidUnicodeSurrogatePair => None,
        }
    }
}
