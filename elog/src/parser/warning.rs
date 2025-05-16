#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum ParserWarningKind {
    #[error("GLOBAL is deprecated in temporary table creation")]
    DeprecatedGlobalTemporaryTable,

    #[error("{0}")]
    ExtendedStringWarning(#[from] ExtendedStringWarning)
}

impl ErrorReport for ParserWarningKind {

    fn sql_state(&self) -> SqlState {
        match self {
            Self::DeprecatedGlobalTemporaryTable => SqlState::Warning,
            Self::ExtendedStringWarning(warn) => warn.sql_state(),
        }
    }

    fn hint(&self) -> Option<Str> {
        match self {
            Self::DeprecatedGlobalTemporaryTable => None,
            Self::ExtendedStringWarning(err) => err.hint(),
        }
    }
}

use crate::extended_string::ExtendedStringWarning;
use crate::sql_state::SqlState;
use crate::ErrorReport;
use pg_basics::Str;
