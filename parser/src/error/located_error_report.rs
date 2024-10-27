#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocatedErrorReport<T> {
    source: T,
    fn_info: &'static FnInfo,
    location: Location,
}

impl<T> LocatedErrorReport<T> {

    pub fn new(source: T, fn_info: &'static FnInfo, location: Location) -> Self {
        Self { source, fn_info, location }
    }

    pub fn source(&self) -> &T {
        &self.source
    }
}

impl<T> HasFnInfo for LocatedErrorReport<T> {
    fn fn_info(&self) -> &'static FnInfo {
        self.fn_info
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

use crate::error::HasLocation;
use postgres_basics::{
    elog::HasFnInfo,
    FnInfo,
    Location
};
use std::error::Error;
use std::fmt::Display;
