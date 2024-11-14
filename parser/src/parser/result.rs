#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ScanErrorKind {
    /// When an unrecoverable error occurs.
    ScanErr(ParserError),
    /// When there are no more tokens.
    Eof(Location),
    /// When the token didn't match.
    NoMatch(Location),
}

impl From<LexerError> for ScanErrorKind {
    fn from(value: LexerError) -> Self {
        ScanErr(value.into())
    }
}

impl From<ParserError> for ScanErrorKind {
    fn from(value: ParserError) -> Self {
        ScanErr(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EofErrorKind {
    NotEof(ParserError),
    Eof(Location),
}

impl From<LexerError> for EofErrorKind {
    fn from(value: LexerError) -> Self {
        NotEof(ParserError::from(value))
    }
}

impl From<ParserError> for EofErrorKind {
    fn from(value: ParserError) -> Self {
        NotEof(value)
    }
}

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
    /// `Eof` and `NoMatch` become `Err(Syntax)`
    fn required(self, fn_info: &'static FnInfo) -> ParseResult<T>;
}

impl<T> Required<T> for ScanResult<T> {
    fn required(self, fn_info: &'static FnInfo) -> ParseResult<T> {
        self.map_err(|err| match err {
            ScanErr(err) => err,
            NoMatch(loc) | ScanEof(loc) => syntax_err(fn_info, loc)
        })
    }
}

impl<T> Required<T> for EofResult<T> {
    fn required(self, fn_info: &'static FnInfo) -> ParseResult<T> {
        self.map_err(|err| match err {
            NotEof(err) => err,
            Eof(loc) => syntax_err(fn_info, loc)
        })
    }
}

pub(super) trait TryMatch<T> {
    /// `Eof` becomes `Err(Syntax)`
    ///
    /// `NoMatch` becomes `Ok(None)`
    fn try_match(self, fn_info: &'static FnInfo) -> ParseResult<Option<T>>;
}

impl<T> TryMatch<T> for ScanResult<T> {
    fn try_match(self, fn_info: &'static FnInfo) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch(_)) => Ok(None),
            Err(ScanEof(loc)) => Err(ParserError::syntax(fn_info, loc)),
            Err(ScanErr(err)) => Err(err),
        }
    }
}

impl<T> TryMatch<T> for EofResult<T> {
    fn try_match(self, fn_info: &'static FnInfo) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(Eof(loc)) => Err(ParserError::syntax(fn_info, loc)),
            Err(NotEof(err)) => Err(err),
        }
    }
}

pub(super) trait Optional<T> {
    /// `Eof` and `NoMatch` become `Ok(None)`
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
    /// Hoists `NoMatch` to `Ok(None)`.
    ///
    /// Usually used when the 1st token is optional,
    /// or there are multiple rules in the production,
    /// but it should still break the whole production on `Eof` and `ParserErr`.
    fn no_match_to_option(self) -> EofResult<Option<T>>;
}

impl<T> ScanResultTrait<T> for ScanResult<T> {
    fn no_match_to_option(self) -> EofResult<Option<T>> {
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

use crate::{
    lexer::LexerError,
    parser::{
        error::syntax_err,
        result::{
            EofErrorKind::{Eof, NotEof},
            ScanErrorKind::{Eof as ScanEof, NoMatch, ScanErr}
        },
        ParseResult,
        ParserError
    }
};
use postgres_basics::{FnInfo, Location};
