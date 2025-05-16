#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocatedErrorReport<T> {
    source: T,
    location: Location,
}

impl<T> LocatedErrorReport<T> {

    pub fn new(source: T, location: Location) -> Self {
        Self { source, location }
    }

    pub fn source(&self) -> &T {
        &self.source
    }
}

impl<T> HasLocation for LocatedErrorReport<T> {
    fn location(&self) -> &Location {
        &self.location
    }
}

impl<T: Error + 'static> Error for LocatedErrorReport<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl<T: Error> Display for LocatedErrorReport<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.source, f)
    }
}

use crate::HasLocation;
use pg_basics::Location;
use std::error::Error;
use std::fmt::Display;
