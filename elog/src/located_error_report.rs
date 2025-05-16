#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocatedErrorReport<T>
where
    T: ErrorReport
{
    source: T,
    location: Location,
}

impl<T> LocatedErrorReport<T>
where
    T: ErrorReport
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

impl<T> Error for LocatedErrorReport<T>
where
    T: ErrorReport + 'static
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl<T> Display for LocatedErrorReport<T>
where
    T: ErrorReport
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let sql_state= self.source.sql_state();
        let source = &self.source;
        writeln!(f, "[{sql_state}] ERROR: {source}")?;

        let position = self.location.range().start + 1;
        write!(f, "Position: {position}")
    }
}

impl<T> HasLocation for LocatedErrorReport<T>
where
    T: ErrorReport
{
    fn location(&self) -> &Location {
        &self.location
    }
}

impl<T> ErrorReport for LocatedErrorReport<T>
where
    T: ErrorReport + 'static
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

impl<T> From<LocatedErrorReport<T>> for Located<T>
where
    T: ErrorReport
{
    fn from(value: LocatedErrorReport<T>) -> Self {
        (value.source, value.location)
    }
}

use crate::sql_state::SqlState;
use crate::ErrorReport;
use crate::HasLocation;
use pg_basics::Str;
use pg_basics::{Located, Location};
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
