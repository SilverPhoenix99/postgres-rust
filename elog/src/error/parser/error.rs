pub type LocatedError = Located<Error>;
pub type Result<T> = core::result::Result<T, Error>;
pub type LocatedResult<T> = core::result::Result<T, LocatedError>;

#[derive(Debug, Default, Clone, Eq, PartialEq, Display)]
pub enum Error {
    /// When a production fails.
    #[default]
    #[display("syntax error")]
    Syntax,

    /// When UESCAPE isn't followed by a simple 1 char string.
    #[display("UESCAPE must be followed by a simple string literal")]
    UescapeDelimiterMissing,

    /// When UESCAPE's delimiter string is invalid (len > 1, or invalid char).
    #[display("invalid Unicode escape character")]
    InvalidUescapeDelimiter,

    /// When the float precision is < 1
    #[display("precision for type float must be at least 1 bit")]
    FloatPrecisionUnderflow(i32),

    /// When the float precision is > 53
    #[display("precision for type float must be less than 54 bits")]
    FloatPrecisionOverflow(i32),

    /// When an identifier is used as an unrecognized role option
    #[display(r#"unrecognized role option "{_0}""#)]
    UnrecognizedRoleOption(Box<str>),

    /// When "UNENCRYPTED PASSWORD" is used as a role option
    #[display("UNENCRYPTED PASSWORD is no longer supported")]
    UnencryptedPassword,

    #[display("improper qualified name (too many dotted names): {_0}")]
    ImproperQualifiedName(NameList),

    #[display("time zone interval must be HOUR or HOUR TO MINUTE")]
    InvalidZoneValue,

    #[display("missing argument")]
    MissingOperatorArgumentType,

    #[display("aggregates cannot have output arguments")]
    AggregateWithOutputParameters,

    #[display(r#"improper use of "*""#)]
    ImproperUseOfStar,

    #[display("frame start cannot be UNBOUNDED FOLLOWING")]
    InvalidUnboundedFollowingFrame,

    #[display("frame starting from following row cannot end with current row")]
    InvalidOffsetFollowingFrame,

    #[display("frame end cannot be UNBOUNDED PRECEDING")]
    InvalidUnboundedPrecedingFrame,

    #[display("frame starting from current row cannot have preceding rows")]
    InvalidCurrentRowFrame,

    #[display("frame starting from following row cannot have preceding rows")]
    InvalidStartFollowingEndPrecedingFrame,

    #[display("type modifier cannot have parameter name")]
    InvalidNamedTypeModifier,

    #[display("type modifier cannot have ORDER BY")]
    InvalidOrderedTypeModifiers,

    #[display("cannot use multiple ORDER BY clauses with WITHIN GROUP")]
    MultipleOrderBy,

    #[display("cannot use DISTINCT with WITHIN GROUP")]
    DistinctWithinGroup,

    #[display("cannot use VARIADIC with WITHIN GROUP")]
    VariadicWithinGroup,

    #[display("unrecognized JSON encoding: {_0}")]
    UnrecognizedJsonEncoding(Str),

    #[display(r#"option name "{_0}" cannot be used in XMLTABLE"#)]
    InvalidXmlTableOptionName(Box<str>),

    #[display(r#"unrecognized column option "{_0}""#)]
    UnrecognizedColumnOption(Box<str>),

    #[display("only one DEFAULT value is allowed")]
    DefaultValueAlreadyDeclared,

    #[display("only one PATH value per column is allowed")]
    PathValueAlreadyDeclared,

    #[display(r#"conflicting or redundant NULL / NOT NULL declarations for column "{_0}""#)]
    ConflictingNullability(Str),

    #[display("only string constants are supported in JSON_TABLE path specification")]
    NonStringJsonTablePathSpec,
}

impl core::error::Error for Error {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NameList(pub QualifiedName);

impl Display for NameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        if self.0.is_empty() {
            return f.write_str("")
        }

        let name = self.0
            .iter()
            .fold(String::new(), |acc, s| acc + "." + s);

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
    UnencryptedPassword => [
        FeatureNotSupported,
        Some("Remove UNENCRYPTED to store the password in encrypted form instead.")
    ],
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
    DefaultValueAlreadyDeclared => [SyntaxError, None],
    PathValueAlreadyDeclared => [SyntaxError, None],
    ConflictingNullability => [SyntaxError, None],
    NonStringJsonTablePathSpec => [FeatureNotSupported, None],
}

use crate::sql_state::SqlState;
use crate::sql_state::SqlState::FeatureNotSupported;
use crate::sql_state::SqlState::InvalidParameterValue;
use crate::sql_state::SqlState::SyntaxError;
use crate::sql_state::SqlState::WindowingError;
use crate::LogMessage;
use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;
use derive_more::Display;
use pg_basics::Located;
use pg_basics::QualifiedName;
use pg_basics::Str;
