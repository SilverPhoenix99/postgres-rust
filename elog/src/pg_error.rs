pub type PgError = LocatedError<PgErrorKind>;

#[inline]
pub fn syntax<T>(location: Location) -> T
where
    PgError: Into<T>
{
    PgError::new(Syntax, location).into()
}

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum PgErrorKind {
    #[error("{0}")] Lexer(#[from] lexer::Error),
    #[error("{0}")] Parser(#[from] ParserErrorKind),
    #[error("{0}")] ExtendedString(#[from] extended_string::Error),
    #[error("{0}")] UnicodeString(#[from] unicode_string::Error),
    #[error("{0}")] RoleSpecError(#[from] role_spec::Error),
}

impl Error for PgErrorKind {
    fn sql_state(&self) -> SqlState {
        match self {
            Self::Lexer(err) => err.sql_state(),
            Self::Parser(err) => err.sql_state(),
            Self::ExtendedString(err ) => err.sql_state(),
            Self::UnicodeString(err) => err.sql_state(),
            Self::RoleSpecError(err) => err.sql_state(),
        }
    }

    fn hint(&self) -> Option<Str> {
        match self {
            Self::Lexer(err) => err.hint(),
            Self::Parser(err) => err.hint(),
            Self::ExtendedString(err) => err.hint(),
            Self::UnicodeString(err) => err.hint(),
            Self::RoleSpecError(err) => err.hint(),
        }
    }

    fn detail(&self) -> Option<Str> {
        match self {
            Self::Lexer(err) => err.detail(),
            Self::Parser(err) => err.detail(),
            Self::ExtendedString(err) => err.detail(),
            Self::UnicodeString(err) => err.detail(),
            Self::RoleSpecError(err) => err.detail(),
        }
    }

    fn detail_log(&self) -> Option<Str> {
        match self {
            Self::Lexer(err) => err.detail_log(),
            Self::Parser(err) => err.detail_log(),
            Self::ExtendedString(err) => err.detail_log(),
            Self::UnicodeString(err) => err.detail_log(),
            Self::RoleSpecError(err) => err.detail_log(),
        }
    }
}

impl From<lexer::LocatedError> for PgError {
    fn from(value: lexer::LocatedError) -> Self {
        let (source, location) = value.into();
        let source = PgErrorKind::Lexer(source);
        Self::new(source, location)
    }
}

impl From<ParserError> for PgError {
    fn from(value: ParserError) -> Self {
        let (source, location) = value.into();
        let source = PgErrorKind::Parser(source);
        Self::new(source, location)
    }
}

use crate::extended_string;
use crate::lexer;
use crate::role_spec;
use crate::unicode_string;
use crate::Error;
use crate::LocatedError;
use crate::ParserError;
use crate::ParserErrorKind;
use crate::ParserErrorKind::Syntax;
use crate::SqlState;
use pg_basics::Location;
use pg_basics::Str;
