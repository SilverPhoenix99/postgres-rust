#[derive(Debug, Copy, Clone, Eq, PartialEq, From, Display)]
pub enum Warning {
    #[display("GLOBAL is deprecated in temporary table creation")]
    DeprecatedGlobalTemporaryTable,
}

impl core::error::Error for Warning {}

impl LogMessage for Warning {

    fn sql_state(&self) -> SqlState {
        match self {
            Self::DeprecatedGlobalTemporaryTable => SqlState::Warning,
        }
    }

    fn hint(&self) -> Option<&str> {
        match self {
            Self::DeprecatedGlobalTemporaryTable => None,
        }
    }
}

use crate::sql_state::SqlState;
use crate::LogMessage;
use derive_more::Display;
use derive_more::From;
