#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EofErrorKind {
    NotEof(PgError),
    Eof(Location),
}

impl_from!(PgError for EofErrorKind::NotEof);

impl From<LexerError> for EofErrorKind {
    fn from(value: LexerError) -> Self {
        Self::NotEof(value.into())
    }
}

impl From<ParserError> for EofErrorKind {
    fn from(value: ParserError) -> Self {
        Self::NotEof(value.into())
    }
}

use pg_basics::impl_from;
use pg_basics::Location;
use pg_elog::LexerError;
use pg_elog::ParserError;
use pg_elog::PgError;
