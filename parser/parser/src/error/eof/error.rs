#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Error {
    NotEof(LocatedError),
    Eof(Location),
}

impl<T> From<T> for Error
where
    T: Into<LocatedError>
{
    fn from(value: T) -> Self {
        Self::NotEof(value.into())
    }
}

use pg_basics::Location;
use pg_elog::LocatedError;
