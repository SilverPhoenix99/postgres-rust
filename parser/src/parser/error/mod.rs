mod parser_error_kind;

pub use parser_error_kind::{NameList, ParserErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParserError(
    LocatedErrorReport<ParserErrorKind>
);

impl ParserError {
    pub fn new(source: ParserErrorKind, fn_info: &'static FnInfo, location: Location) -> Self {
        Self(LocatedErrorReport::new(source, fn_info, location))
    }

    pub fn syntax(fn_info: &'static FnInfo, location: Location) -> Self {
        Self::new(Syntax, fn_info, location)
    }

    pub fn source(&self) -> &ParserErrorKind {
        self.0.source()
    }
}

impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        let source = value.source().into();
        Self::new(source, value.fn_info(), value.location().clone())
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

pub(super) fn syntax_err(fn_info: &'static FnInfo, location: Location) -> ParserError {
    ParserError::syntax(fn_info, location)
}

use crate::{
    error::{HasLocation, LocatedErrorReport},
    lexer::LexerError,
    parser::ParserErrorKind::Syntax
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
