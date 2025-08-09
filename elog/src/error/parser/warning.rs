#[derive(Debug, Copy, Clone, Eq, PartialEq, From, Display)]
pub enum Warning {
    #[display("GLOBAL is deprecated in temporary table creation")]
    DeprecatedGlobalTemporaryTable,

    #[display("{_0}")]
    ExtendedStringWarning(extended_string::Warning)
}

impl core::error::Error for Warning {}

impl LogMessage for Warning {

    fn sql_state(&self) -> SqlState {
        match self {
            Self::DeprecatedGlobalTemporaryTable => SqlState::Warning,
            Self::ExtendedStringWarning(warn) => warn.sql_state(),
        }
    }

    fn hint(&self) -> Option<&str> {
        match self {
            Self::DeprecatedGlobalTemporaryTable => None,
            Self::ExtendedStringWarning(err) => err.hint(),
        }
    }
}

use crate::extended_string;
use crate::sql_state::SqlState;
use crate::LogMessage;
use derive_more::Display;
use derive_more::From;
