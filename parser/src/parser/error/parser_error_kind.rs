#[derive(Debug, Default, Clone, Eq, PartialEq, thiserror::Error)]
pub enum ParserErrorKind {
    /// When a production fails.
    #[default]
    #[error("syntax error")]
    Syntax,

    /// Still a syntax error, but thrown by the lexer
    #[error("{0}")]
    Lexer(#[from] LexerErrorKind),

    /// When UESCAPE isn't followed by a simple 1 char string.
    #[error("UESCAPE must be followed by a simple string literal")]
    UescapeDelimiterMissing,

    /// When UESCAPE's delimiter string is invalid (len > 1, or invalid char).
    #[error("invalid Unicode escape character")]
    InvalidUescapeDelimiter,

    #[error("{0}")]
    ExtendedString(#[from] ExtendedStringError),

    #[error("{0}")]
    UnicodeString(#[from] UnicodeStringError),

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

    #[error("improper qualified name (too many dotted names): {0}")]
    ImproperQualifiedName(NameList),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NameList(pub QualifiedName);

impl Display for NameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.join("."))
    }
}

impl HasSqlState for ParserErrorKind {
    fn sql_state(&self) -> SqlState {
        match self {
            ExtendedString(err) => err.sql_state(),
            FloatPrecisionOverflow(_) => InvalidParameterValue,
            FloatPrecisionUnderflow(_) => InvalidParameterValue,
            ForbiddenRoleSpec(_) => ReservedName,
            InvalidUescapeDelimiter => SyntaxError,
            Lexer(err) => err.sql_state(),
            ReservedRoleSpec(_) => ReservedName,
            Syntax => SyntaxError,
            UescapeDelimiterMissing => SyntaxError,
            UnencryptedPassword => FeatureNotSupported,
            UnicodeString(err) => err.sql_state(),
            UnrecognizedRoleOption(_) => SyntaxError,
            ImproperQualifiedName(_) => SyntaxError,
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
            Lexer(err) => err.hint(),
            ReservedRoleSpec(_) => None,
            Syntax => None,
            UescapeDelimiterMissing => None,
            UnencryptedPassword => Some("Remove UNENCRYPTED to store the password in encrypted form instead.".into()),
            UnicodeString(err) => err.hint(),
            UnrecognizedRoleOption(_) => None,
            ImproperQualifiedName(_) => None,
        }
    }

    fn detail(&self) -> Option<Cow<'static, str>> {
        match self {
            ExtendedString(err) => err.detail(),
            FloatPrecisionOverflow(_) => None,
            FloatPrecisionUnderflow(_) => None,
            ForbiddenRoleSpec(_) => None,
            InvalidUescapeDelimiter => None,
            Lexer(err) => err.detail(),
            ReservedRoleSpec(_) => None,
            Syntax => None,
            UescapeDelimiterMissing => None,
            UnencryptedPassword => None,
            UnicodeString(err) => err.detail(),
            UnrecognizedRoleOption(_) => None,
            ImproperQualifiedName(_) => None,
        }
    }

    fn detail_log(&self) -> Option<Cow<'static, str>> {
        match self {
            ExtendedString(err) => err.detail_log(),
            FloatPrecisionOverflow(_) => None,
            FloatPrecisionUnderflow(_) => None,
            ForbiddenRoleSpec(_) => None,
            InvalidUescapeDelimiter => None,
            Lexer(err) => err.detail_log(),
            ReservedRoleSpec(_) => None,
            Syntax => None,
            UescapeDelimiterMissing => None,
            UnencryptedPassword => None,
            UnicodeString(err) => err.detail_log(),
            UnrecognizedRoleOption(_) => None,
            ImproperQualifiedName(_) => None,
        }
    }
}

use crate::lexer::LexerErrorKind;
use crate::parser::QualifiedName;
use crate::string_decoders::{
    ExtendedStringError,
    UnicodeStringError,
};
use postgres_basics::{
    elog::{ErrorReport, HasSqlState},
    sql_state::SqlState::{
        self,
        FeatureNotSupported,
        InvalidParameterValue,
        ReservedName,
        SyntaxError,
    },
};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use ParserErrorKind::*;
