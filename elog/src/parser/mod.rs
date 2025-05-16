mod parser_error_kind;
mod warning;

pub use self::{
    parser_error_kind::{NameList, ParserErrorKind},
    warning::ParserWarningKind,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParserError(
    LocatedErrorReport<ParserErrorKind>
);

impl ParserError {
    pub fn new<T: Into<ParserErrorKind>>(source: T, location: Location) -> Self {
        Self(LocatedErrorReport::new(source.into(), location))
    }

    pub fn syntax(location: Location) -> Self {
        Self::new(Syntax, location)
    }

    pub fn source(&self) -> &ParserErrorKind {
        self.0.source()
    }
}

impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        let source = ParserErrorKind::from(value.source());
        Self::new(source, value.location().clone())
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Error::source(&self.0)
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sql_state= self.sql_state();
        let source = self.0.source();
        let position = self.0.location().range().start + 1;
        writeln!(f, "[{sql_state}] ERROR: {source}")?;
        write!(f, "Position: {position}")
    }
}

impl HasSqlState for ParserError {
    fn sql_state(&self) -> SqlState {
        self.0.source().sql_state()
    }
}

impl ErrorReport for ParserError {
    fn hint(&self) -> Option<Cow<'static, str>> {
        self.0.source().hint()
    }

    fn detail(&self) -> Option<Cow<'static, str>> {
        self.0.source().detail()
    }

    fn detail_log(&self) -> Option<Cow<'static, str>> {
        self.0.source().detail_log()
    }
}

impl HasLocation for ParserError {
    fn location(&self) -> &Location {
        self.0.location()
    }
}

use crate::lexer::LexerError;
use crate::sql_state::SqlState;
use crate::ErrorReport;
use crate::HasLocation;
use crate::HasSqlState;
use crate::LocatedErrorReport;
use pg_basics::Location;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use ParserErrorKind::Syntax;
