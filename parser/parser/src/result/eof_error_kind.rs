#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EofErrorKind {
    NotEof(PgError),
    Eof(Location),
}

impl<T> From<T> for EofErrorKind
where
    T: Into<PgError>
{
    fn from(value: T) -> Self {
        Self::NotEof(value.into())
    }
}

use pg_basics::Location;
use pg_elog::PgError;
