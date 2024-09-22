use crate::lexer::LexerError;
use crate::string_decoders::{BitStringError, ExtendedStringError, UnicodeStringError};
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use ParserError::*;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ParserError {
    /// When a production fails.
    #[default]
    Syntax,

    /// Still a syntax error, but thrown by the lexer
    Lexer(LexerError),

    /// When UESCAPE isn't followed by a simple 1 char string.
    // "UESCAPE must be followed by a simple string literal"
    UescapeDelimiterMissing,

    /// When UESCAPE's delimiter string is invalid (len > 1, or invalid char).
    // "invalid Unicode escape character"
    InvalidUescapeDelimiter,

    /// When the input has an invalid UTF-8 char
    Utf8(Utf8Error),

    ExtendedString(ExtendedStringError),

    UnicodeString(UnicodeStringError),

    /// When a bit string exceeds `VARBITMAXLEN`
    BitStringTooLong,

    /// When a char is not a valid binary digit in a bit string.
    InvalidBinaryDigit,

    /// When a Unicode escape (e.g., `'\XXXX'`) is invalid.
    InvalidUnicodeCodepoint,

    /// When a UTF-16 surrogate pair is invalid.
    /// E.g., the 2nd codepoint of the pair is invalid.
    InvalidUnicodeSurrogatePair,

    /// When using the '' escape in an unsafe string
    NonstandardUseOfEscapeCharacter,

    /// When "none" or "public" was incorrectly used as a role.
    // #[error(r#"role name "{0}" is reserved"#)]
    ReservedRoleSpec(&'static str),

    /// When a role is disallowed
    // #[error(r#""#)]
    ForbiddenRoleSpec(&'static str),

    /// When the float precision is < 1
    // #[error("precision for type float must be at least 1 bit")]
    FloatPrecisionUnderflow(i32),

    /// When the float precision is > 53
    // #[error("precision for type float must be less than 54 bits")]
    FloatPrecisionOverflow(i32),
}

impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        Lexer(value)
    }
}

impl From<Utf8Error> for ParserError {
    fn from(value: Utf8Error) -> Self {
        Utf8(value)
    }
}

impl From<FromUtf8Error> for ParserError {
    fn from(value: FromUtf8Error) -> Self {
        Utf8(value.utf8_error())
    }
}

impl From<BitStringError> for ParserError {
    fn from(value: BitStringError) -> Self {
        match value {
            BitStringError::BitStringTooLong => BitStringTooLong,
            BitStringError::InvalidBinaryDigit => InvalidBinaryDigit,
        }
    }
}
