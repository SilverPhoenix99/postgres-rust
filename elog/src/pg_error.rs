pub type PgError = LocatedMessage<Error>;

#[inline]
pub fn syntax<T>(location: Location) -> T
where
    PgError: Into<T>
{
    PgError::new(Syntax, location).into()
}

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("{0}")] Lexer(#[from] lexer::Error),
    #[error("{0}")] Parser(#[from] parser::Error),
    #[error("{0}")] ExtendedString(#[from] extended_string::Error),
    #[error("{0}")] UnicodeString(#[from] unicode_string::Error),
    #[error("{0}")] RoleSpecError(#[from] role_spec::Error),
}

impl LogMessage for Error {
    fn sql_state(&self) -> SqlState {
        match self {
            Lexer(err) => err.sql_state(),
            Parser(err) => err.sql_state(),
            Self::ExtendedString(err ) => err.sql_state(),
            Self::UnicodeString(err) => err.sql_state(),
            Self::RoleSpecError(err) => err.sql_state(),
        }
    }

    fn hint(&self) -> Option<Str> {
        match self {
            Lexer(err) => err.hint(),
            Parser(err) => err.hint(),
            Self::ExtendedString(err) => err.hint(),
            Self::UnicodeString(err) => err.hint(),
            Self::RoleSpecError(err) => err.hint(),
        }
    }

    fn detail(&self) -> Option<Str> {
        match self {
            Lexer(err) => err.detail(),
            Parser(err) => err.detail(),
            Self::ExtendedString(err) => err.detail(),
            Self::UnicodeString(err) => err.detail(),
            Self::RoleSpecError(err) => err.detail(),
        }
    }

    fn detail_log(&self) -> Option<Str> {
        match self {
            Lexer(err) => err.detail_log(),
            Parser(err) => err.detail_log(),
            Self::ExtendedString(err) => err.detail_log(),
            Self::UnicodeString(err) => err.detail_log(),
            Self::RoleSpecError(err) => err.detail_log(),
        }
    }
}

impl From<lexer::LocatedError> for PgError {
    fn from(value: lexer::LocatedError) -> Self {
        let (source, location) = value.into();
        let source = Lexer(source);
        Self::new(source, location)
    }
}

impl From<parser::LocatedError> for PgError {
    fn from(value: parser::LocatedError) -> Self {
        let (source, location) = value.into();
        let source = Parser(source);
        Self::new(source, location)
    }
}

use self::Error::Lexer;
use self::Error::Parser;
use crate::extended_string;
use crate::lexer;
use crate::parser;
use crate::parser::Error::Syntax;
use crate::role_spec;
use crate::unicode_string;
use crate::LocatedMessage;
use crate::LogMessage;
use crate::SqlState;
use pg_basics::Location;
use pg_basics::Str;
