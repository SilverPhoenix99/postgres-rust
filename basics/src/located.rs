#[derive(Debug, Clone, PartialEq, Eq, From, Into)]
#[from((T, Location))]
#[into((T, Location))]
pub struct Located<T>(pub T, pub Location);

impl<T> Located<T> {

    pub fn new<U>(source: U, location: Location) -> Self
    where
        U: Into<T>,
    {
        Self(source.into(), location)
    }

    pub fn source(&self) -> &T {
        &self.0
    }

    pub fn location(&self) -> &Location {
        &self.1
    }

    pub fn into_source(self) -> T {
        self.0
    }
}

impl<T> Display for Located<T>
where
    T: Display + 'static
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{source}\nPosition: {position}",
            source = self.0,
            position = self.1.range().start + 1
        )
    }
}

impl<T> Error for Located<T>
where
    T: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

pub trait IntoLocated
where
    Self: Sized
{
    fn into_located<T>(self, location: Location) -> Located<T>
    where
        T: From<Self>,
    {
        Located::new(self, location)
    }

    fn at_location(self, location: Location) -> Located<Self> {
        Located(self, location)
    }
}

impl<T> IntoLocated for T
where
    Self: Sized
{}

use crate::Location;
use core::error::Error;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use derive_more::From;
use derive_more::Into;
