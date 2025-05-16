#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ScanErrorKind {
    /// When an unrecoverable error occurs.
    ScanErr(ParserError),
    /// When there are no more tokens.
    Eof(Location),
    /// When the token didn't match.
    NoMatch(Location),
}

impl_from!(LexerError for ScanErrorKind::ScanErr);
impl_from!(ParserError for ScanErrorKind::ScanErr);

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
use pg_elog::lexer::LexerError;
use pg_elog::parser::ParserError;
