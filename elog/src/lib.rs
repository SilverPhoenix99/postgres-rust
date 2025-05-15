pub mod extended_string;
pub mod lexer;
pub mod parser;
pub mod sql_state;

mod error_report;
mod has_location;
mod has_sql_state;
mod located_error_report;
mod log_level;
mod parse_report;
mod role_spec_error;
mod simple_error_report;
mod sql_report;
mod unicode_string_error;

pub use self::{
    error_report::ErrorReport,
    has_location::HasLocation,
    has_sql_state::HasSqlState,
    located_error_report::LocatedErrorReport,
    log_level::LogLevel,
    parse_report::ParseReport,
    role_spec_error::RoleSpecError,
    simple_error_report::SimpleErrorReport,
    sql_report::SqlReport,
    unicode_string_error::UnicodeStringError,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PgError {
    Lexer(),
    Parser(),
}
