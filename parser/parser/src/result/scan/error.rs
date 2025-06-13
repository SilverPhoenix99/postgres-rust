#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Error {
    /// When an unrecoverable error occurs.
    ScanErr(PgError),
    /// When there are no more tokens.
    Eof(Location),
    /// When the token didn't match.
    NoMatch(Location),
}

impl<T> From<T> for Error
where
    T: Into<PgError>
{
    fn from(value: T) -> Self {
        ScanErr(value.into())
    }
}

impl From<eof::Error> for Error {
    fn from(value: eof::Error) -> Self {
        match value {
            eof::Error::NotEof(err) => ScanErr(err),
            eof::Error::Eof(loc) => Self::Eof(loc)
        }
    }
}

use crate::eof;
use crate::scan::Error::ScanErr;
use pg_basics::Location;
use pg_elog::PgError;
