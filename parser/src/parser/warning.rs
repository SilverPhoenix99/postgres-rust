#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum ParserWarningKind {
    #[error("GLOBAL is deprecated in temporary table creation")]
    DeprecatedGlobalTemporaryTable,

    #[error("nonstandard use of escape in a string literal")]
    NonstandardEscape,

    #[error(r"nonstandard use of \' in a string literal")]
    NonstandardQuoteEscape,

    #[error(r"nonstandard use of \\ in a string literal")]
    NonstandardBackslashEscape,
}

impl HasSqlState for ParserWarningKind {
    fn sql_state(&self) -> SqlState {
        match self {
            Self::DeprecatedGlobalTemporaryTable => SqlState::Warning,
            Self::NonstandardEscape
            | Self::NonstandardQuoteEscape
            | Self::NonstandardBackslashEscape => SqlState::NonstandardUseOfEscapeCharacter,
        }
    }
}

impl ErrorReport for ParserWarningKind {
    fn hint(&self) -> Option<Cow<'static, str>> {
        match self {
            Self::DeprecatedGlobalTemporaryTable => None,
            Self::NonstandardEscape => Some(r"Use the escape string syntax for escapes, e.g., E'\r\n'.".into()),
            Self::NonstandardQuoteEscape => {
                Some("Use '' to write quotes in strings, or use the escape string syntax (E'...').".into())
            },
            Self::NonstandardBackslashEscape => {
                Some(r"Use the escape string syntax for backslashes, e.g., E'\\'.".into())
            },
        }
    }
}

impl From<ExtendedStringWarning> for ParserWarningKind {
    fn from(value: ExtendedStringWarning) -> Self {
        match value {
            ExtendedStringWarning::NonstandardEscape => Self::NonstandardEscape,
            ExtendedStringWarning::NonstandardQuoteEscape => Self::NonstandardQuoteEscape,
            ExtendedStringWarning::NonstandardBackslashEscape => Self::NonstandardBackslashEscape,
        }
    }
}

use crate::string_decoders::ExtendedStringWarning;
use postgres_basics::elog::ErrorReport;
use postgres_basics::elog::HasSqlState;
use postgres_basics::sql_state::SqlState;
use std::borrow::Cow;
