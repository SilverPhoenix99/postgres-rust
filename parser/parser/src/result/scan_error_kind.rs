#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ScanErrorKind {
    /// When an unrecoverable error occurs.
    ScanErr(PgError),
    /// When there are no more tokens.
    Eof(Location),
    /// When the token didn't match.
    NoMatch(Location),
}

impl_from!(PgError for ScanErrorKind::ScanErr);

impl From<lexer::LocatedError> for ScanErrorKind {
    fn from(value: lexer::LocatedError) -> Self {
        ScanErr(value.into())
    }
}

impl From<ParserError> for ScanErrorKind {
    fn from(value: ParserError) -> Self {
        ScanErr(value.into())
    }
}

impl From<EofErrorKind> for ScanErrorKind {
    fn from(value: EofErrorKind) -> Self {
        use EofErrorKind::*;
        match value {
            NotEof(err) => ScanErr(err),
            Eof(loc) => Self::Eof(loc)
        }
    }
}

use crate::result::EofErrorKind;
use crate::result::ScanErrorKind::ScanErr;
use pg_basics::impl_from;
use pg_basics::Location;
use pg_elog::lexer;
use pg_elog::ParserError;
use pg_elog::PgError;
