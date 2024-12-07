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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EofErrorKind {
    NotEof(ParserError),
    Eof(Location),
}

impl_from!(LexerError for EofErrorKind::NotEof);
impl_from!(ParserError for EofErrorKind::NotEof);

impl From<EofErrorKind> for ScanErrorKind {
    fn from(value: EofErrorKind) -> Self {
        match value {
            NotEof(err) => ScanErr(err),
            Eof(loc) => Self::Eof(loc)
        }
    }
}

pub(super) type ScanResult<T> = Result<T, ScanErrorKind>;
pub(super) type EofResult<T> = Result<T, EofErrorKind>;

pub(super) trait Required<T> {
    /// See [`required()`](crate::parser::combinators::required::required).
    fn required(self) -> ParseResult<T>;
}

impl<T> Required<T> for ScanResult<T> {
    fn required(self) -> ParseResult<T> {
        self.map_err(|err| match err {
            ScanErr(err) => err,
            NoMatch(loc) | ScanEof(loc) => syntax_err(loc)
        })
    }
}

impl<T> Required<T> for EofResult<T> {
    fn required(self) -> ParseResult<T> {
        self.map_err(|err| match err {
            NotEof(err) => err,
            Eof(loc) => syntax_err(loc)
        })
    }
}

pub(super) trait TryMatch<T> {
    /// See [`try_match()`](crate::parser::combinators::try_match::try_match).
    fn try_match(self) -> ParseResult<Option<T>>;
}

impl<T> TryMatch<T> for ScanResult<T> {
    fn try_match(self) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch(_)) => Ok(None),
            Err(ScanEof(loc)) => Err(ParserError::syntax(loc)),
            Err(ScanErr(err)) => Err(err),
        }
    }
}

impl<T> TryMatch<T> for EofResult<T> {
    fn try_match(self) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(Eof(loc)) => Err(ParserError::syntax(loc)),
            Err(NotEof(err)) => Err(err),
        }
    }
}

pub(super) trait Optional<T> {
    /// See [`optional()`](crate::parser::combinators::optional::optional).
    fn optional(self) -> ParseResult<Option<T>>;
}

impl<T> Optional<T> for ScanResult<T> {
    fn optional(self) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch(_) | ScanEof(_)) => Ok(None),
            Err(ScanErr(err)) => Err(err),
        }
    }
}

impl<T> Optional<T> for EofResult<T> {
    fn optional(self) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(Eof(_)) => Ok(None),
            Err(NotEof(err)) => Err(err),
        }
    }
}

pub(super) trait ScanResultTrait<T> {
    /// See [`maybe_match()`](crate::parser::combinators::maybe_match::maybe_match).
    fn maybe_match(self) -> EofResult<Option<T>>;
}

impl<T> ScanResultTrait<T> for ScanResult<T> {
    fn maybe_match(self) -> EofResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch(_)) => Ok(None),
            Err(ScanEof(loc)) => Err(Eof(loc)),
            Err(ScanErr(err)) => Err(NotEof(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO
}

use crate::lexer::LexerError;
use crate::parser::error::syntax_err;
use crate::parser::result::EofErrorKind::Eof;
use crate::parser::result::EofErrorKind::NotEof;
use crate::parser::result::ScanErrorKind::Eof as ScanEof;
use crate::parser::result::ScanErrorKind::NoMatch;
use crate::parser::result::ScanErrorKind::ScanErr;
use crate::parser::ParseResult;
use crate::parser::ParserError;
use postgres_basics::impl_from;
use postgres_basics::Location;
