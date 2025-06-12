pub mod extended_string {
    pub use crate::error::extended_string::error::*;
    pub use crate::error::extended_string::warning::*;
}

pub mod lexer {
    pub use crate::error::lexer::*;
}

pub mod role_spec {
    pub use crate::error::role_spec::*;
}

mod error;
mod has_location;
mod log_level;
mod parser;
mod pg_error;
mod sql_state;
mod unicode_string_error;

pub use self::{
    error::located_error::LocatedError,
    error::Error,
    has_location::HasLocation,
    log_level::LogLevel,
    parser::{NameList, ParserError, ParserErrorKind, ParserWarningKind},
    pg_error::{syntax, PgError, PgErrorKind},
    sql_state::{SqlState, SqlStateCategory, UnknownSqlState},
    unicode_string_error::UnicodeStringError,
};
