pub trait LogMessage: Error {

    fn sql_state(&self) -> SqlState;

    #[inline(always)]
    fn hint(&self) -> Option<&str> {
        None
    }

    #[inline(always)]
    fn detail(&self) -> Option<&str> {
        None
    }

    #[inline(always)]
    fn detail_log(&self) -> Option<&str> {
        None
    }
}

use crate::SqlState;
use core::error::Error;
