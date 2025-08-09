pub trait HasLocation {
    fn location(&self) -> &Location;
}

impl<T> HasLocation for Located<T>
where
    T: LogMessage
{
    fn location(&self) -> &Location {
        &self.1
    }
}

use crate::LogMessage;
use pg_basics::Located;
use pg_basics::Location;
