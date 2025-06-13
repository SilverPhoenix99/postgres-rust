pub mod extended_string {
    pub use crate::error::extended_string::error::*;
    pub use crate::error::extended_string::warning::*;
}

pub mod lexer {
    pub use crate::error::lexer::*;
}

pub mod parser {
    pub use crate::error::parser::error::*;
    pub use crate::error::parser::warning::*;
}

pub mod role_spec {
    pub use crate::error::role_spec::*;
}

pub mod unicode_string {
    pub use crate::error::unicode_string::*;
}

mod error;
mod has_location;
mod log_level;
mod log_message;
mod sql_state;

pub use self::{
    error::located_message::LocatedMessage,
    error::{syntax, Error, LocatedError, LocatedResult},
    has_location::HasLocation,
    log_level::LogLevel,
    log_message::LogMessage,
    sql_state::{SqlState, SqlStateCategory, UnknownSqlState},
};
