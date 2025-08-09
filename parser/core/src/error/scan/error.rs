#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    /// When an unrecoverable error occurs.
    ScanErr(pg_elog::LocatedError),
    /// When there are no more tokens.
    Eof(Location),
    /// When the token didn't match.
    NoMatch(Location),
}

impl<T> From<Located<T>> for Error
where
    T: Into<pg_elog::Error>
{
    fn from(Located(source, location): Located<T>) -> Self {
        ScanErr(Located::new(source, location))
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
use pg_basics::Located;
use pg_basics::Location;
