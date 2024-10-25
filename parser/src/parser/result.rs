#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ScanErrorKind {
    /// When an unrecoverable error occurs.
    ScanErr(ParserErrorKind),
    /// When there are no more tokens.
    Eof,
    /// When the token didn't match.
    NoMatch,
}

impl Default for ScanErrorKind {
    fn default() -> Self {
        ScanErr(Default::default())
    }
}

impl From<LexerError> for ScanErrorKind {
    fn from(value: LexerError) -> Self {
        ScanErr(value.into())
    }
}

impl From<ParserErrorKind> for ScanErrorKind {
    fn from(value: ParserErrorKind) -> Self {
        ScanErr(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EofErrorKind {
    NotEof(ParserErrorKind),
    Eof,
}

impl Default for EofErrorKind {
    fn default() -> Self {
        NotEof(Default::default())
    }
}

impl From<LexerError> for EofErrorKind {
    fn from(value: LexerError) -> Self {
        NotEof(value.into())
    }
}

impl From<ParserErrorKind> for EofErrorKind {
    fn from(value: ParserErrorKind) -> Self {
        NotEof(value)
    }
}

impl From<EofErrorKind> for ParserErrorKind {
    fn from(value: EofErrorKind) -> Self {
        match value {
            NotEof(err) => err,
            _ => Default::default()
        }
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
    fn required(self) -> ParseResult<T>;
}

impl<T> Required<T> for ScanResult<T> {
    fn required(self) -> ParseResult<T> {
        self.map_err(|err| match err {
            ScanErr(err) => err,
            _ => Default::default()
        })
    }
}

impl<T> Required<T> for EofResult<T> {
    fn required(self) -> ParseResult<T> {
        self.map_err(|err| match err {
            NotEof(err) => err,
            _ => Default::default()
        })
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

use crate::{
    lexer::LexerError,
    parser::{
        result::{
            EofErrorKind::NotEof,
            ScanErrorKind::{NoMatch, ScanErr}
        },
        ParseResult,
        ParserErrorKind
    }
};
