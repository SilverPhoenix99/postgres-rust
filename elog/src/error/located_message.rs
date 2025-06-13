#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocatedMessage<T>
where
    T: Error
{
    source: T,
    location: Location,
}

impl<T> LocatedMessage<T>
where
    T: Error
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
    T: Error + 'static
{
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl<T> Display for LocatedMessage<T>
where
    T: Error
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {

        let sql_state= self.source.sql_state();
        let source = &self.source;
        writeln!(f, "[{sql_state}] ERROR: {source}")?;

        let position = self.location.range().start + 1;
        write!(f, "Position: {position}")
    }
}

impl<T> HasLocation for LocatedMessage<T>
where
    T: Error
{
    fn location(&self) -> &Location {
        &self.location
    }
}

impl<T> Error for LocatedMessage<T>
where
    T: Error + 'static
{
    #[inline(always)]
    fn sql_state(&self) -> SqlState {
        self.source.sql_state()
    }

    #[inline(always)]
    fn hint(&self) -> Option<Str> {
        self.source.hint()
    }

    #[inline(always)]
    fn detail(&self) -> Option<Str> {
        self.source.detail()
    }

    #[inline(always)]
    fn detail_log(&self) -> Option<Str> {
        self.source.detail_log()
    }
}

impl<T> From<LocatedMessage<T>> for Located<T>
where
    T: Error
{
    fn from(value: LocatedMessage<T>) -> Self {
        (value.source, value.location)
    }
}

use crate::sql_state::SqlState;
use crate::Error;
use crate::HasLocation;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use pg_basics::Located;
use pg_basics::Location;
use pg_basics::Str;
