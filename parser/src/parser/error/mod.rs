mod parser_error_kind;

pub use parser_error_kind::ParserErrorKind;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParserError {
    report: SimpleErrorReport<ParserErrorKind>,
    location: Location
}

impl ParserError {
    pub fn source(&self) -> ParserErrorKind {
        self.report.source().clone()
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Error::source(&self.report)
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sql_state= self.sql_state();
        let source = self.source();
        let position = self.location.range().start + 1;
        writeln!(f, "[{sql_state}] ERROR: {source}")?;
        write!(f, "Position: {position}")
    }
}

impl HasSqlState for ParserError {
    fn sql_state(&self) -> SqlState {
        self.source().sql_state()
    }
}

impl HasFnInfo for ParserError {
    fn fn_info(&self) -> &'static FnInfo {
        self.report.fn_info()
    }
}

impl HasLocation for ParserError {
    fn location(&self) -> &Location {
        &self.location
    }
}

impl ErrorReport for ParserError {
    fn hint(&self) -> Option<Cow<'static, str>> {
        self.source().hint()
    }

    fn detail(&self) -> Option<Cow<'static, str>> {
        self.source().detail()
    }

    fn detail_log(&self) -> Option<Cow<'static, str>> {
        self.source().detail_log()
    }
}

impl SqlReport for ParserError {}

impl ParseReport for ParserError {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct PartialParserError {
    report: SimpleErrorReport<ParserErrorKind>,
    location: Option<Location>,
}

impl PartialParserError {
    pub(super) fn new(source: ParserErrorKind, fn_info: &'static FnInfo) -> Self {
        Self {
            report: SimpleErrorReport::new(source, fn_info),
            location: None
        }
    }

    pub(super) fn with_location(self, location: Location) -> Self {
        Self {
            report: self.report,
            location: Some(location)
        }
    }

    pub(super) fn syntax(fn_info: &'static FnInfo) -> Self {
        Self::new(Default::default(), fn_info)
    }

    pub(super) fn report(&self) -> &SimpleErrorReport<ParserErrorKind> {
        &self.report
    }

    pub(super) fn location(&self) -> &Option<Location> {
        &self.location
    }

    pub(super) fn into_parser_err(self, default_location: Location) -> ParserError {
        ParserError {
            report: self.report,
            location: self.location.unwrap_or(default_location)
        }
    }
}

impl From<LexerError> for PartialParserError {
    fn from(value: LexerError) -> Self {
        Self {
            report: SimpleErrorReport::new(value.source().into(), value.fn_info()),
            location: Some(value.location().clone())
        }
    }
}

use crate::error::{HasLocation, ParseReport};
use crate::lexer::LexerError;
use postgres_basics::{elog::{ErrorReport, HasFnInfo, HasSqlState, SimpleErrorReport, SqlReport}, sql_state::SqlState, FnInfo, Location};
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};
