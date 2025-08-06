#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotEof(pg_elog::LocatedError),
    Eof(Location),
}

impl<T> From<T> for Error
where
    T: Into<pg_elog::LocatedError>
{
    fn from(value: T) -> Self {
        Self::NotEof(value.into())
    }
}

use pg_basics::Location;
