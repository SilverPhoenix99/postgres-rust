#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ScanErrorKind {
    /// When an unrecoverable error occurs.
    ScanErr(PartialParserError),
    /// When there are no more tokens.
    Eof,
    /// When the token didn't match.
    NoMatch,
}

impl From<LexerError> for ScanErrorKind {
    fn from(value: LexerError) -> Self {
        ScanErr(value.into())
    }
}

impl From<PartialParserError> for ScanErrorKind {
    fn from(value: PartialParserError) -> Self {
        ScanErr(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EofErrorKind {
    NotEof(PartialParserError),
    Eof,
}

impl From<LexerError> for EofErrorKind {
    fn from(value: LexerError) -> Self {
        NotEof(value.into())
    }
}

impl From<PartialParserError> for EofErrorKind {
    fn from(value: PartialParserError) -> Self {
        NotEof(value)
    }
}

impl From<EofErrorKind> for ScanErrorKind {
    fn from(value: EofErrorKind) -> Self {
        match value {
            NotEof(err) => ScanErr(err),
            EofErrorKind::Eof => Self::Eof
        }
    }
}

pub(super) type ScanResult<T> = Result<T, ScanErrorKind>;
pub(super) type EofResult<T> = Result<T, EofErrorKind>;

pub(super) trait Required<T> {
    fn required(self, fn_info: &'static FnInfo) -> ParseResult<T>;
}

impl<T> Required<T> for ScanResult<T> {
    fn required(self, fn_info: &'static FnInfo) -> ParseResult<T> {
        self.map_err(|err| match err {
            ScanErr(err) => err,
            _ => PartialParserError::syntax(fn_info)
        })
    }
}

impl<T> Required<T> for EofResult<T> {
    fn required(self, fn_info: &'static FnInfo) -> ParseResult<T> {
        self.map_err(|err| match err {
            NotEof(err) => err,
            _ => PartialParserError::syntax(fn_info)
        })
    }
}

pub(super) trait TryMatch<T> {
    fn try_match(self, fn_info: &'static FnInfo) -> ParseResult<Option<T>>;
}

impl<T> TryMatch<T> for ScanResult<T> {
    fn try_match(self, fn_info: &'static FnInfo) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch) => Ok(None),
            Err(ScanErrorKind::Eof) => Err(PartialParserError::syntax(fn_info)),
            Err(ScanErr(err)) => Err(err),
        }
    }
}

impl<T> TryMatch<T> for EofResult<T> {
    fn try_match(self, fn_info: &'static FnInfo) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(EofErrorKind::Eof) => Err(PartialParserError::syntax(fn_info)),
            Err(NotEof(err)) => Err(err),
        }
    }
}

pub(super) trait Optional<T> {
    fn optional(self) -> ParseResult<Option<T>>;
}

impl<T> Optional<T> for ScanResult<T> {
    fn optional(self) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch | ScanErrorKind::Eof) => Ok(None),
            Err(ScanErr(err)) => Err(err),
        }
    }
}

impl<T> Optional<T> for EofResult<T> {
    fn optional(self) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(EofErrorKind::Eof) => Ok(None),
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
            Err(NoMatch) => Ok(None),
            Err(ScanErrorKind::Eof) => Err(EofErrorKind::Eof),
            Err(ScanErr(err)) => Err(NotEof(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO
}

use crate::parser::error::PartialParserError;
use crate::{
    lexer::LexerError,
    parser::{
        result::{
            EofErrorKind::NotEof,
            ScanErrorKind::{NoMatch, ScanErr}
        },
        ParseResult
    }
};
use postgres_basics::FnInfo;
