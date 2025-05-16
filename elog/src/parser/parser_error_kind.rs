#[derive(Debug, Default, Clone, Eq, PartialEq, thiserror::Error)]
pub enum ParserErrorKind {
    /// When a production fails.
    #[default]
    #[error("syntax error")]
    Syntax,

    /// When UESCAPE isn't followed by a simple 1 char string.
    #[error("UESCAPE must be followed by a simple string literal")]
    UescapeDelimiterMissing,

    /// When UESCAPE's delimiter string is invalid (len > 1, or invalid char).
    #[error("invalid Unicode escape character")]
    InvalidUescapeDelimiter,

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

    #[error(r#"improper use of "*""#)]
    ImproperUseOfStar
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

impl ErrorReport for ParserErrorKind {

    fn sql_state(&self) -> SqlState {
        match self {
            FloatPrecisionOverflow(_) => InvalidParameterValue,
            FloatPrecisionUnderflow(_) => InvalidParameterValue,
            InvalidUescapeDelimiter => SyntaxError,
            Syntax => SyntaxError,
            UescapeDelimiterMissing => SyntaxError,
            UnencryptedPassword => FeatureNotSupported,
            UnrecognizedRoleOption(_) => SyntaxError,
            ImproperQualifiedName(_) => SyntaxError,
            InvalidZoneValue => SyntaxError,
            MissingOperatorArgumentType => SyntaxError,
            AggregateWithOutputParameters => FeatureNotSupported,
            ImproperUseOfStar => SyntaxError,
        }
    }

    fn hint(&self) -> Option<Str> {
        match self {
            UnencryptedPassword => Some("Remove UNENCRYPTED to store the password in encrypted form instead.".into()),
            FloatPrecisionOverflow(_) => None,
            FloatPrecisionUnderflow(_) => None,
            InvalidUescapeDelimiter => None,
            Syntax => None,
            UescapeDelimiterMissing => None,
            UnrecognizedRoleOption(_) => None,
            ImproperQualifiedName(_) => None,
            InvalidZoneValue => None,
            MissingOperatorArgumentType => Some("Use NONE to denote the missing argument of a unary operator.".into()),
            AggregateWithOutputParameters => None,
            ImproperUseOfStar => None,
        }
    }
}

use crate::sql_state::SqlState;
use crate::sql_state::SqlState::FeatureNotSupported;
use crate::sql_state::SqlState::InvalidParameterValue;
use crate::sql_state::SqlState::SyntaxError;
use crate::ErrorReport;
use pg_basics::QualifiedName;
use pg_basics::Str;
use std::fmt::Display;
use std::fmt::Formatter;
use ParserErrorKind::*;
