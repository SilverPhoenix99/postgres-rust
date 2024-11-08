mod parser_error_kind;

pub use parser_error_kind::{NameList, ParserErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParserError(
    LocatedErrorReport<ParserErrorKind>
);

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

impl HasFnInfo for ParserError {
    fn fn_info(&self) -> &'static FnInfo {
        self.0.fn_info()
    }
}

impl HasLocation for ParserError {
    fn location(&self) -> &Location {
        self.0.location()
    }
}

macro_rules! syntax_err {
    ($fn_name:expr) => { $crate::parser::error::PartialParserError::syntax(fn_info!($fn_name)).into() };
}
pub(super) use syntax_err;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct PartialParserError {
    source: ParserErrorKind,
    fn_info: &'static FnInfo,
    location: Option<Location>,
}

impl PartialParserError {
    pub fn new(source: ParserErrorKind, fn_info: &'static FnInfo) -> Self {
        Self {
            source,
            fn_info,
            location: None
        }
    }

    pub fn with_location(self, location: Location) -> Self {
        Self {
            location: Some(location),
            ..self
        }
    }

    pub fn syntax(fn_info: &'static FnInfo) -> Self {
        Self::new(Default::default(), fn_info)
    }

    pub fn source(&self) -> &ParserErrorKind {
        &self.source
    }

    pub fn location(&self) -> &Option<Location> {
        &self.location
    }

    pub(super) fn into_parser_err(self, default_location: Location) -> ParserError {
        let location = self.location.unwrap_or(default_location);
        let report = LocatedErrorReport::new(self.source, self.fn_info, location);
        ParserError(report)
    }
}

impl HasFnInfo for PartialParserError {
    fn fn_info(&self) -> &'static FnInfo {
        self.fn_info
    }
}

impl From<LexerError> for PartialParserError {
    fn from(value: LexerError) -> Self {
        Self {
            source: value.source().into(),
            fn_info: value.fn_info(),
            location: Some(value.location().clone())
        }
    }
}

use crate::{
    error::{HasLocation, LocatedErrorReport},
    lexer::LexerError
};
use postgres_basics::{
    elog::{ErrorReport, HasFnInfo, HasSqlState},
    sql_state::SqlState,
    FnInfo,
    Location
};
use std::{
    borrow::Cow,
    error::Error,
    fmt::{Debug, Display, Formatter}
};
