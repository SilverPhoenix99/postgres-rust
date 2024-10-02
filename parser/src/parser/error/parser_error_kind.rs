#[derive(Debug, Default, Clone, Eq, PartialEq, thiserror::Error)]
pub enum ParserErrorKind {
    /// When a production fails.
    #[default]
    #[error("syntax error")]
    Syntax,

    /// Still a syntax error, but thrown by the lexer
    #[error("{0}")]
    Lexer(#[from] LexerError),

    /// When UESCAPE isn't followed by a simple 1 char string.
    #[error("UESCAPE must be followed by a simple string literal")]
    UescapeDelimiterMissing,

    /// When UESCAPE's delimiter string is invalid (len > 1, or invalid char).
    #[error("invalid Unicode escape character")]
    InvalidUescapeDelimiter,

    /// When the input has an invalid UTF-8 char
    #[error("TODO: invalid UTF-8")] // TODO
    Utf8(Utf8Error),

    #[error("{0}")]
    ExtendedString(#[from] ExtendedStringError),

    #[error("{0}")]
    UnicodeString(#[from] UnicodeStringError),

    /// When a bit string exceeds `VARBITMAXLEN`
    #[error(
        "bit string length exceeds the maximum allowed ({})",
        crate::string_decoders::VARBITMAXLEN
    )]
    BitStringTooLong,

    /// When a char is not a valid binary digit in a bit string.
    #[error("TODO: invalid binary digit")] // TODO
    InvalidBinaryDigit,

    /// When a Unicode escape (e.g., `'\XXXX'`) is invalid.
    #[error("TODO: invalid Unicode codepoint")] // TODO
    InvalidUnicodeCodepoint,

    /// When a UTF-16 surrogate pair is invalid.
    /// E.g., the 2nd codepoint of the pair is invalid.
    #[error("TODO: invalid Unicode surrogate pair")] // TODO
    InvalidUnicodeSurrogatePair,

    /// When using the '' escape in an unsafe string
    #[error("TODO: non-standard use of escape character")] // TODO
    NonstandardUseOfEscapeCharacter,

    /// When "none" or "public" was incorrectly used as a role.
    #[error(r#"role name "{0}" is reserved"#)]
    ReservedRoleSpec(&'static str),

    /// When a role is disallowed
    #[error(r"{0} cannot be used as a role name here")]
    ForbiddenRoleSpec(&'static str),

    /// When the float precision is < 1
    #[error("precision for type float must be at least 1 bit")]
    FloatPrecisionUnderflow(i32),

    /// When the float precision is > 53
    #[error("precision for type float must be less than 54 bits")]
    FloatPrecisionOverflow(i32),

    /// When an identifier is used as an unrecognized role option
    #[error(r#"unrecognized role option "{0}""#)]
    UnrecognizedRoleOption(String),

    /// When "UNENCRYPTED PASSWORD" is used as a role option
    #[error("UNENCRYPTED PASSWORD is no longer supported")]
    UnencryptedPassword,
}

impl HasSqlState for ParserErrorKind {
    fn sql_state(&self) -> SqlState {
        match self {
            ExtendedString(err) => err.sql_state(),
            FloatPrecisionOverflow(_) => SqlState::Error(InvalidParameterValue),
            FloatPrecisionUnderflow(_) => SqlState::Error(InvalidParameterValue),
            ForbiddenRoleSpec(_) => SqlState::Error(ReservedName),
            InvalidUescapeDelimiter => SqlState::Error(SyntaxError),
            InvalidUnicodeCodepoint => {todo!()},
            InvalidUnicodeSurrogatePair => {todo!()},
            Lexer(err) => err.sql_state(),
            NonstandardUseOfEscapeCharacter => {todo!()},
            ReservedRoleSpec(_) => SqlState::Error(ReservedName),
            Self::BitStringTooLong => SqlState::Error(ProgramLimitExceeded),
            Self::InvalidBinaryDigit => {todo!()},
            Utf8(_) => {todo!()},
            Syntax => SqlState::Error(SyntaxError),
            UescapeDelimiterMissing => SqlState::Error(SyntaxError),
            UnencryptedPassword => SqlState::Error(FeatureNotSupported),
            UnicodeString(err) => err.sql_state(),
            UnrecognizedRoleOption(_) => SqlState::Error(SyntaxError),
        }
    }
}

impl ErrorReport for ParserErrorKind {
    fn hint(&self) -> Option<Cow<'static, str>> {
        match self {
            ExtendedString(err) => err.hint(),
            FloatPrecisionOverflow(_) => None,
            FloatPrecisionUnderflow(_) => None,
            ForbiddenRoleSpec(_) => None,
            InvalidUescapeDelimiter => None,
            InvalidUnicodeCodepoint => {todo!()},
            InvalidUnicodeSurrogatePair => {todo!()},
            Lexer(err) => err.hint(),
            NonstandardUseOfEscapeCharacter => {todo!()},
            ReservedRoleSpec(_) => None,
            Self::BitStringTooLong => {todo!()},
            Self::InvalidBinaryDigit => {todo!()},
            Utf8(_) => {todo!()},
            Syntax => None,
            UescapeDelimiterMissing => None,
            UnencryptedPassword => Some("Remove UNENCRYPTED to store the password in encrypted form instead.".into()),
            UnicodeString(err) => err.hint(),
            UnrecognizedRoleOption(_) => None,
        }
    }

    fn detail(&self) -> Option<Cow<'static, str>> {
        match self {
            ExtendedString(err) => err.detail(),
            FloatPrecisionOverflow(_) => None,
            FloatPrecisionUnderflow(_) => None,
            ForbiddenRoleSpec(_) => None,
            InvalidUescapeDelimiter => None,
            InvalidUnicodeCodepoint => {todo!()},
            InvalidUnicodeSurrogatePair => {todo!()},
            Lexer(err) => err.detail(),
            NonstandardUseOfEscapeCharacter => {todo!()},
            ReservedRoleSpec(_) => None,
            Self::BitStringTooLong => {todo!()},
            Self::InvalidBinaryDigit => {todo!()},
            Utf8(_) => {todo!()},
            Syntax => None,
            UescapeDelimiterMissing => None,
            UnencryptedPassword => None,
            UnicodeString(err) => err.detail(),
            UnrecognizedRoleOption(_) => None,
        }
    }

    fn detail_log(&self) -> Option<Cow<'static, str>> {
        match self {
            ExtendedString(err) => err.detail_log(),
            FloatPrecisionOverflow(_) => None,
            FloatPrecisionUnderflow(_) => None,
            ForbiddenRoleSpec(_) => None,
            InvalidUescapeDelimiter => None,
            InvalidUnicodeCodepoint => {todo!()},
            InvalidUnicodeSurrogatePair => {todo!()},
            Lexer(err) => err.detail_log(),
            NonstandardUseOfEscapeCharacter => {todo!()},
            ReservedRoleSpec(_) => None,
            Self::BitStringTooLong => {todo!()},
            Self::InvalidBinaryDigit => {todo!()},
            Utf8(_) => {todo!()},
            Syntax => None,
            UescapeDelimiterMissing => None,
            UnencryptedPassword => None,
            UnicodeString(err) => err.detail_log(),
            UnrecognizedRoleOption(_) => None,
        }
    }
}

impl From<Utf8Error> for ParserErrorKind {
    fn from(value: Utf8Error) -> Self {
        Utf8(value)
    }
}

impl From<FromUtf8Error> for ParserErrorKind {
    fn from(value: FromUtf8Error) -> Self {
        Utf8(value.utf8_error())
    }
}

impl From<BitStringError> for ParserErrorKind {
    fn from(value: BitStringError) -> Self {
        match value {
            BitStringTooLong => Self::BitStringTooLong,
            InvalidBinaryDigit => Self::InvalidBinaryDigit,
        }
    }
}

use crate::lexer::LexerError;
use crate::string_decoders::{
    BitStringError,
    BitStringError::{BitStringTooLong, InvalidBinaryDigit},
    ExtendedStringError,
    UnicodeStringError
};
use postgres_basics::{
    elog::{ErrorReport, HasSqlState},
    sql_state::ErrorSqlState::{
        FeatureNotSupported,
        InvalidParameterValue,
        ProgramLimitExceeded,
        ReservedName,
        SyntaxError
    },
    sql_state::SqlState,
};
use std::borrow::Cow;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use ParserErrorKind::*;
