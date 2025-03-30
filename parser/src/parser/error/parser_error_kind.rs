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
    UnrecognizedRoleOption(Box<str>),

    /// When "UNENCRYPTED PASSWORD" is used as a role option
    #[error("UNENCRYPTED PASSWORD is no longer supported")]
    UnencryptedPassword,

    #[error("improper qualified name (too many dotted names): {0}")]
    ImproperQualifiedName(NameList),

    #[error("time zone interval must be HOUR or HOUR TO MINUTE")]
    InvalidZoneValue,

    #[error("missing argument")]
    MissingOperatorArgumentType,
    
    #[error("aggregates cannot have output arguments")]
    AggregateWithOutputParameters,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NameList(pub QualifiedName);

impl Display for NameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        if self.0.is_empty() {
            return f.write_str("")
        }

        let name = self.0
            .iter()
            .fold(String::new(), |acc, s| acc + "." + s.as_ref());

        let name = &name[1..];

        f.write_str(name)
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
            InvalidZoneValue => SyntaxError,
            MissingOperatorArgumentType => SyntaxError,
            AggregateWithOutputParameters => FeatureNotSupported,
        }
    }
}

impl ErrorReport for ParserErrorKind {
    fn hint(&self) -> Option<Cow<'static, str>> {
        match self {
            ExtendedString(err) => err.hint(),
            Lexer(err) => err.hint(),
            UnicodeString(err) => err.hint(),
            UnencryptedPassword => Some("Remove UNENCRYPTED to store the password in encrypted form instead.".into()),
            FloatPrecisionOverflow(_) => None,
            FloatPrecisionUnderflow(_) => None,
            ForbiddenRoleSpec(_) => None,
            InvalidUescapeDelimiter => None,
            ReservedRoleSpec(_) => None,
            Syntax => None,
            UescapeDelimiterMissing => None,
            UnrecognizedRoleOption(_) => None,
            ImproperQualifiedName(_) => None,
            InvalidZoneValue => None,
            MissingOperatorArgumentType => Some("Use NONE to denote the missing argument of a unary operator.".into()),
            AggregateWithOutputParameters => None,
        }
    }

    fn detail(&self) -> Option<Cow<'static, str>> {
        match self {
            ExtendedString(err) => err.detail(),
            Lexer(err) => err.detail(),
            UnicodeString(err) => err.detail(),
            FloatPrecisionOverflow(_) => None,
            FloatPrecisionUnderflow(_) => None,
            ForbiddenRoleSpec(_) => None,
            InvalidUescapeDelimiter => None,
            ReservedRoleSpec(_) => None,
            Syntax => None,
            UescapeDelimiterMissing => None,
            UnencryptedPassword => None,
            UnrecognizedRoleOption(_) => None,
            ImproperQualifiedName(_) => None,
            InvalidZoneValue => None,
            MissingOperatorArgumentType => None,
            AggregateWithOutputParameters => None,
        }
    }

    fn detail_log(&self) -> Option<Cow<'static, str>> {
        match self {
            ExtendedString(err) => err.detail_log(),
            Lexer(err) => err.detail_log(),
            UnicodeString(err) => err.detail_log(),
            FloatPrecisionOverflow(_) => None,
            FloatPrecisionUnderflow(_) => None,
            ForbiddenRoleSpec(_) => None,
            InvalidUescapeDelimiter => None,
            ReservedRoleSpec(_) => None,
            Syntax => None,
            UescapeDelimiterMissing => None,
            UnencryptedPassword => None,
            UnrecognizedRoleOption(_) => None,
            ImproperQualifiedName(_) => None,
            InvalidZoneValue => None,
            MissingOperatorArgumentType => None,
            AggregateWithOutputParameters => None,
        }
    }
}

use crate::lexer::LexerErrorKind;
use crate::parser::ast_node::QualifiedName;
use crate::string_decoders::ExtendedStringError;
use crate::string_decoders::UnicodeStringError;
use postgres_basics::elog::ErrorReport;
use postgres_basics::elog::HasSqlState;
use postgres_basics::sql_state::SqlState;
use postgres_basics::sql_state::SqlState::FeatureNotSupported;
use postgres_basics::sql_state::SqlState::InvalidParameterValue;
use postgres_basics::sql_state::SqlState::ReservedName;
use postgres_basics::sql_state::SqlState::SyntaxError;
use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;
use ParserErrorKind::*;
