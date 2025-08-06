#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocatedMessage<T>
where
    T: LogMessage
{
    source: T,
    location: Location,
}

impl<T> LocatedMessage<T>
where
    T: LogMessage
{
    pub fn new<S: Into<T>>(source: S, location: Location) -> Self {
        Self {
            source: source.into(),
            location
        }
    }

    pub fn source(&self) -> &T {
        &self.source
    }
}

impl<T> core::error::Error for LocatedMessage<T>
where
    T: LogMessage + 'static
{
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl<T> Display for LocatedMessage<T>
where
    T: LogMessage
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        let sql_state= self.source.sql_state();
        let source = &self.source;
        writeln!(f, "[{sql_state}] ERROR: {source}")?;

        let position = self.location.range().start + 1;
        write!(f, "Position: {position}")
    }
}

impl<T> HasLocation for LocatedMessage<T>
where
    T: LogMessage
{
    fn location(&self) -> &Location {
        &self.location
    }
}

impl<T> LogMessage for LocatedMessage<T>
where
    T: LogMessage + 'static
{
    fn sql_state(&self) -> SqlState {
        self.source.sql_state()
    }

    fn hint(&self) -> Option<&str> {
        self.source.hint()
    }

    fn detail(&self) -> Option<&str> {
        self.source.detail()
    }

    fn detail_log(&self) -> Option<&str> {
        self.source.detail_log()
    }
}

impl<T> From<LocatedMessage<T>> for Located<T>
where
    T: LogMessage
{
    fn from(value: LocatedMessage<T>) -> Self {
        (value.source, value.location)
    }
}

use crate::sql_state::SqlState;
use crate::HasLocation;
use crate::LogMessage;
use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;
use pg_basics::Located;
use pg_basics::Location;
