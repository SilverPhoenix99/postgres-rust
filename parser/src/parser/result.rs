#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ScanErrorKind {
    /// When an unrecoverable error occurs.
    ParserErr(ParserErrorKind),
    /// When there are no more tokens.
    Eof,
    /// When the token didn't match.
    NoMatch,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum EofErrorKind {
    ParserErr(ParserErrorKind),
    Eof,
}

impl From<ParserErrorKind> for ScanErrorKind {
    fn from(value: ParserErrorKind) -> Self {
        Self::ParserErr(value)
    }
}

impl Default for ScanErrorKind {
    fn default() -> Self {
        Self::ParserErr(ParserErrorKind::default())
    }
}

pub(crate) trait ScanResultTrait<T> {

    /// When both `Eof` and `NoMatch` are considered syntax errors.
    ///
    /// Hoists both `Eof` and `NoMatch` to `ParserErrorKind::default()`.
    fn required(self) -> Result<T, ParserErrorKind>;

    /// When it's fine if the token doesn't match or is missing.
    ///
    /// Hoists both `Eof` and `NoMatch` to `Ok(None)`.
    fn optional(self) -> Result<Option<T>, ParserErrorKind>;

    /// Hoists `NoMatch` to `Ok(None)`.
    ///
    /// Usually used when the 1st token is optional,
    /// or there are multiple rules in the production,
    /// but it should still break the whole production on `Eof` and `ParserErr`.
    fn no_match_to_option(self) -> EofResult<Option<T>>;
}

pub(crate) type ScanResult<T> = Result<T, ScanErrorKind>;

impl<T> ScanResultTrait<T> for ScanResult<T> {

    fn required(self) -> Result<T, ParserErrorKind> {
        self.map_err(|err| match err {
            ScanErrorKind::ParserErr(err) => err,
            _ => ParserErrorKind::default()
        })
    }

    fn optional(self) -> Result<Option<T>, ParserErrorKind> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(ScanErrorKind::Eof | ScanErrorKind::NoMatch) => Ok(None),
            Err(ScanErrorKind::ParserErr(err)) => Err(err),
        }
    }

    fn no_match_to_option(self) -> EofResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(ScanErrorKind::NoMatch) => Ok(None),
            Err(ScanErrorKind::Eof) => Err(EofErrorKind::Eof),
            Err(ScanErrorKind::ParserErr(err)) => Err(EofErrorKind::ParserErr(err)),
        }
    }
}

impl From<EofErrorKind> for ScanErrorKind {
    fn from(value: EofErrorKind) -> Self {
        match value {
            EofErrorKind::Eof => Self::Eof,
            EofErrorKind::ParserErr(err) => Self::ParserErr(err),
        }
    }
}

impl Default for EofErrorKind {
    fn default() -> Self {
        Self::ParserErr(ParserErrorKind::default())
    }
}

impl From<ParserErrorKind> for EofErrorKind {
    fn from(value: ParserErrorKind) -> Self {
        Self::ParserErr(value)
    }
}

impl From<EofErrorKind> for ParserErrorKind {
    /// Hoists `Eof` to `ParserErrorKind::default()`.
    fn from(value: EofErrorKind) -> Self {
        match value {
            EofErrorKind::ParserErr(err) => err,
            _ => ParserErrorKind::default()
        }
    }
}

pub(crate) trait EofResultTrait<T> {
    /// When `Eof` is considered a `Syntax` error.
    ///
    /// Hoists `Eof` to `ParserErrorKind::default()`.
    fn required(self) -> Result<T, ParserErrorKind>;
}

pub(crate) type EofResult<T> = Result<T, EofErrorKind>;

impl<T> EofResultTrait<T> for EofResult<T> {
    fn required(self) -> Result<T, ParserErrorKind> {
        self.map_err(ParserErrorKind::from)
    }
}

#[cfg(test)]
mod tests {
    // TODO
}

use crate::parser::ParserErrorKind;
