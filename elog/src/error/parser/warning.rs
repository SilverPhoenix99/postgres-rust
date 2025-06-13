#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Warning {
    #[error("GLOBAL is deprecated in temporary table creation")]
    DeprecatedGlobalTemporaryTable,

    #[error("{0}")]
    ExtendedStringWarning(#[from] extended_string::Warning)
}

impl LogMessage for Warning {

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

use crate::extended_string;
use crate::sql_state::SqlState;
use crate::LogMessage;
use pg_basics::Str;
