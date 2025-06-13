#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ScanErrorKind {
    /// When an unrecoverable error occurs.
    ScanErr(PgError),
    /// When there are no more tokens.
    Eof(Location),
    /// When the token didn't match.
    NoMatch(Location),
}

impl<T> From<T> for ScanErrorKind
where
    T: Into<PgError>
{
    fn from(value: T) -> Self {
        ScanErr(value.into())
    }
}

impl From<EofErrorKind> for ScanErrorKind {
    fn from(value: EofErrorKind) -> Self {
        match value {
            EofErrorKind::NotEof(err) => ScanErr(err),
            EofErrorKind::Eof(loc) => Self::Eof(loc)
        }
    }
}

use crate::result::EofErrorKind;
use crate::result::ScanErrorKind::ScanErr;
use pg_basics::Location;
use pg_elog::PgError;
