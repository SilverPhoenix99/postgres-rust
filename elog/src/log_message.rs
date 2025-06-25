pub trait LogMessage: Error {

    fn sql_state(&self) -> SqlState;

    fn hint(&self) -> Option<&str> {
        None
    }

    fn detail(&self) -> Option<&str> {
        None
    }

    fn detail_log(&self) -> Option<&str> {
        None
    }
}

use crate::SqlState;
use core::error::Error;
