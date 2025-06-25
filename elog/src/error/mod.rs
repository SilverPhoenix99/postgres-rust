pub(super) mod extended_string;
pub(super) mod lexer;
pub(super) mod located_message;
pub(super) mod parser;
pub(super) mod role_spec;
pub(super) mod unicode_string;

pub type LocatedError = LocatedMessage<Error>;
pub type LocatedResult<T> = Result<T, LocatedError>;

pub fn syntax<T>(location: Location) -> T
where
    LocatedError: Into<T>
{
    LocatedError::new(Syntax, location).into()
}

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("{0}")] Lexer(#[from] lexer::Error),
    #[error("{0}")] Parser(#[from] parser::error::Error),
    #[error("{0}")] ExtendedString(#[from] extended_string::error::Error),
    #[error("{0}")] UnicodeString(#[from] unicode_string::Error),
    #[error("{0}")] Role(#[from] role_spec::Error),
}

impl LogMessage for Error {
    fn sql_state(&self) -> SqlState {
        match self {
            Lexer(err) => err.sql_state(),
            Parser(err) => err.sql_state(),
            Self::ExtendedString(err ) => err.sql_state(),
            Self::UnicodeString(err) => err.sql_state(),
            Role(err) => err.sql_state(),
        }
    }

    fn hint(&self) -> Option<&str> {
        match self {
            Lexer(err) => err.hint(),
            Parser(err) => err.hint(),
            Self::ExtendedString(err) => err.hint(),
            Self::UnicodeString(err) => err.hint(),
            Role(err) => err.hint(),
        }
    }

    fn detail(&self) -> Option<&str> {
        match self {
            Lexer(err) => err.detail(),
            Parser(err) => err.detail(),
            Self::ExtendedString(err) => err.detail(),
            Self::UnicodeString(err) => err.detail(),
            Role(err) => err.detail(),
        }
    }

    fn detail_log(&self) -> Option<&str> {
        match self {
            Lexer(err) => err.detail_log(),
            Parser(err) => err.detail_log(),
            Self::ExtendedString(err) => err.detail_log(),
            Self::UnicodeString(err) => err.detail_log(),
            Role(err) => err.detail_log(),
        }
    }
}

impl From<lexer::LocatedError> for LocatedError {
    fn from(value: lexer::LocatedError) -> Self {
        let (source, location) = value.into();
        let source = Lexer(source);
        Self::new(source, location)
    }
}

impl From<parser::error::LocatedError> for LocatedError {
    fn from(value: parser::error::LocatedError) -> Self {
        let (source, location) = value.into();
        let source = Parser(source);
        Self::new(source, location)
    }
}

impl From<role_spec::LocatedError> for LocatedError {
    fn from(value: role_spec::LocatedError) -> Self {
        let (source, location) = value.into();
        let source = Role(source);
        Self::new(source, location)
    }
}

use self::Error::Lexer;
use self::Error::Parser;
use self::Error::Role;
use crate::parser::Error::Syntax;
use crate::LocatedMessage;
use crate::LogMessage;
use crate::SqlState;
use pg_basics::Location;
