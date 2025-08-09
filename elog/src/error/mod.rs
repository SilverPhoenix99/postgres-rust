pub(super) mod extended_string;
pub(super) mod lexer;
pub(super) mod parser;
pub(super) mod role_spec;
pub(super) mod unicode_string;

pub type Result<T> = core::result::Result<T, Error>;
pub type LocatedError = Located<Error>;
pub type LocatedResult<T> = core::result::Result<T, LocatedError>;

#[derive(Debug, Clone, Eq, PartialEq, From, Display, Error)]
pub enum Error {
    #[display("{_0}")] Lexer(lexer::Error),
    #[display("{_0}")] Parser(parser::error::Error),
    #[display("{_0}")] ExtendedString(extended_string::error::Error),
    #[display("{_0}")] UnicodeString(unicode_string::Error),
    #[display("{_0}")] Role(role_spec::Error),
}

impl LogMessage for Error {
    fn sql_state(&self) -> SqlState {
        match self {
            Lexer(err) => err.sql_state(),
            Parser(err) => err.sql_state(),
            ExtendedString(err ) => err.sql_state(),
            UnicodeString(err) => err.sql_state(),
            Role(err) => err.sql_state(),
        }
    }

    fn hint(&self) -> Option<&str> {
        match self {
            Lexer(err) => err.hint(),
            Parser(err) => err.hint(),
            ExtendedString(err) => err.hint(),
            UnicodeString(err) => err.hint(),
            Role(err) => err.hint(),
        }
    }

    fn detail(&self) -> Option<&str> {
        match self {
            Lexer(err) => err.detail(),
            Parser(err) => err.detail(),
            ExtendedString(err) => err.detail(),
            UnicodeString(err) => err.detail(),
            Role(err) => err.detail(),
        }
    }

    fn detail_log(&self) -> Option<&str> {
        match self {
            Lexer(err) => err.detail_log(),
            Parser(err) => err.detail_log(),
            ExtendedString(err) => err.detail_log(),
            UnicodeString(err) => err.detail_log(),
            Role(err) => err.detail_log(),
        }
    }
}

use crate::Error::ExtendedString;
use crate::Error::Lexer;
use crate::Error::Parser;
use crate::Error::Role;
use crate::Error::UnicodeString;
use crate::LogMessage;
use crate::SqlState;
use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use pg_basics::Located;
