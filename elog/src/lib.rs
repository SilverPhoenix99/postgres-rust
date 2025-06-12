pub mod lexer {
    pub use crate::error::lexer::*;
}

mod error;
mod extended_string;
mod has_location;
mod log_level;
mod parser;
mod pg_error;
mod role_spec_error;
mod sql_state;
mod unicode_string_error;

pub use self::{
    error::{Error, LocatedError},
    extended_string::{ExtendedStringError, ExtendedStringWarning},
    has_location::HasLocation,
    log_level::LogLevel,
    parser::{NameList, ParserError, ParserErrorKind, ParserWarningKind},
    pg_error::{syntax, PgError, PgErrorKind},
    role_spec_error::{RoleSpecError, RoleSpecErrorKind},
    sql_state::{SqlState, SqlStateCategory, UnknownSqlState},
    unicode_string_error::UnicodeStringError,
};
