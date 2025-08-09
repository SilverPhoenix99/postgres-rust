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

impl<T> LogMessage for Located<T>
where
    T: LogMessage + 'static
{
    fn sql_state(&self) -> SqlState {
        self.0.sql_state()
    }

    fn hint(&self) -> Option<&str> {
        self.0.hint()
    }

    fn detail(&self) -> Option<&str> {
        self.0.detail()
    }

    fn detail_log(&self) -> Option<&str> {
        self.0.detail_log()
    }
}

use crate::SqlState;
use core::error::Error;
use pg_basics::Located;
