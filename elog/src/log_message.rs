pub trait LogMessage: Error {

    fn sql_state(&self) -> SqlState;

    #[inline(always)]
    fn hint(&self) -> Option<Str> {
        None
    }

    #[inline(always)]
    fn detail(&self) -> Option<Str> {
        None
    }

    #[inline(always)]
    fn detail_log(&self) -> Option<Str> {
        None
    }
}

use crate::SqlState;
use core::error::Error;
use pg_basics::Str;
