pub type LocatedError = LocatedMessage<Error>;
pub type Result<T> = core::result::Result<T, Error>;
pub type LocatedResult<T> = core::result::Result<T, LocatedError>;

#[derive(Debug, Default, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Error {
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
    ImproperUseOfStar,

    #[error("frame start cannot be UNBOUNDED FOLLOWING")]
    InvalidUnboundedFollowingFrame,

    #[error("frame starting from following row cannot end with current row")]
    InvalidOffsetFollowingFrame,

    #[error("frame end cannot be UNBOUNDED PRECEDING")]
    InvalidUnboundedPrecedingFrame,

    #[error("frame starting from current row cannot have preceding rows")]
    InvalidCurrentRowFrame,

    #[error("frame starting from following row cannot have preceding rows")]
    InvalidStartFollowingEndPrecedingFrame,

    #[error("type modifier cannot have parameter name")]
    InvalidNamedTypeModifier,

    #[error("type modifier cannot have ORDER BY")]
    InvalidOrderedTypeModifiers,

    #[error("cannot use multiple ORDER BY clauses with WITHIN GROUP")]
    MultipleOrderBy,

    #[error("cannot use DISTINCT with WITHIN GROUP")]
    DistinctWithinGroup,

    #[error("cannot use VARIADIC with WITHIN GROUP")]
    VariadicWithinGroup,

    #[error("unrecognized JSON encoding: {}", .0.as_ref())]
    UnrecognizedJsonEncoding(Str),

    #[error(r#"option name "{0}" cannot be used in XMLTABLE"#)]
    InvalidXmlTableOptionName(Box<str>),

    #[error(r#"unrecognized column option "{0}""#)]
    UnrecognizedColumnOption(Box<str>),
}

impl Error {
    pub fn at(self, location: Location) -> LocatedError {
        LocatedError::new(self, location)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NameList(pub QualifiedName);

impl Display for NameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

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

macro_rules! impl_log_message {
    (
        $(
            $variant:ident => [$state:path, $hint:expr]
        ),+
        $(,)?
    ) => {
        impl LogMessage for Error {

            fn sql_state(&self) -> SqlState {
                match self {
                    $(
                        Self::$variant { .. } => $state,
                    )+
                }
            }

            fn hint(&self) -> Option<&str> {
                match self {
                    $(
                        Self::$variant { .. } => $hint,
                    )+
                }
            }
        }
    };
}

impl_log_message! {
    Syntax => [SyntaxError, None],
    UescapeDelimiterMissing => [SyntaxError, None],
    InvalidUescapeDelimiter => [SyntaxError, None],
    FloatPrecisionUnderflow => [InvalidParameterValue, None],
    FloatPrecisionOverflow => [InvalidParameterValue, None],
    UnrecognizedRoleOption => [SyntaxError, None],
    UnencryptedPassword => [FeatureNotSupported, Some("Remove UNENCRYPTED to store the password in encrypted form instead.")],
    ImproperQualifiedName => [SyntaxError, None],
    InvalidZoneValue => [SyntaxError, None],
    MissingOperatorArgumentType => [SyntaxError, Some("Use NONE to denote the missing argument of a unary operator.")],
    AggregateWithOutputParameters => [FeatureNotSupported, None],
    ImproperUseOfStar => [SyntaxError, None],
    InvalidUnboundedFollowingFrame => [WindowingError, None],
    InvalidOffsetFollowingFrame => [WindowingError, None],
    InvalidUnboundedPrecedingFrame => [WindowingError, None],
    InvalidCurrentRowFrame => [WindowingError, None],
    InvalidStartFollowingEndPrecedingFrame => [WindowingError, None],
    InvalidNamedTypeModifier => [SyntaxError, None],
    InvalidOrderedTypeModifiers => [SyntaxError, None],
    MultipleOrderBy => [SyntaxError, None],
    DistinctWithinGroup => [SyntaxError, None],
    VariadicWithinGroup => [SyntaxError, None],
    UnrecognizedJsonEncoding => [InvalidParameterValue, None],
    InvalidXmlTableOptionName => [SyntaxError, None],
    UnrecognizedColumnOption => [SyntaxError, None],
}

use crate::sql_state::SqlState;
use crate::sql_state::SqlState::FeatureNotSupported;
use crate::sql_state::SqlState::InvalidParameterValue;
use crate::sql_state::SqlState::SyntaxError;
use crate::sql_state::SqlState::WindowingError;
use crate::LocatedMessage;
use crate::LogMessage;
use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;
use pg_basics::Location;
use pg_basics::QualifiedName;
use pg_basics::Str;
