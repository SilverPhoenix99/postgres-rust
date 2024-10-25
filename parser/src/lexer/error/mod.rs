mod lexer_error_kind;

pub use lexer_error_kind::LexerErrorKind;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LexerError {
    report: SimpleErrorReport<LexerErrorKind>,
    location: Location,
}

impl LexerError {

    #[inline(always)]
    pub fn new(source: LexerErrorKind, fn_info: &'static FnInfo, location: Location) -> Self {
        Self { 
            report: SimpleErrorReport::new(source, fn_info),
            location
        }
    }

    #[inline(always)]
    pub fn source(&self) -> LexerErrorKind {
        *self.report.source()
    }
}

impl Error for LexerError {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Error::source(&self.report)
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sql_state= self.sql_state();
        let source = self.source();
        let position = self.location.range().start + 1;
        writeln!(f, "[{sql_state}] ERROR: {source}")?;
        write!(f, "Position: {position}")
    }
}

impl HasSqlState for LexerError {
    #[inline(always)]
    fn sql_state(&self) -> SqlState {
        self.source().sql_state()
    }
}

impl HasFnInfo for LexerError {
    fn fn_info(&self) -> &'static FnInfo {
        self.report.fn_info()
    }
}

impl HasLocation for LexerError {
    #[inline(always)]
    fn location(&self) -> &Location {
        &self.location
    }
}

impl ErrorReport for LexerError {
    #[inline(always)]
    fn hint(&self) -> Option<Cow<'static, str>> {
        self.source().hint()
    }

    #[inline(always)]
    fn detail(&self) -> Option<Cow<'static, str>> {
        self.source().detail()
    }

    #[inline(always)]
    fn detail_log(&self) -> Option<Cow<'static, str>> {
        self.source().detail_log()
    }
}

impl SqlReport for LexerError {}

impl ParseReport for LexerError {}

use crate::error::{HasLocation, ParseReport};
use postgres_basics::elog::{ErrorReport, HasFnInfo, HasSqlState, SimpleErrorReport, SqlReport};
use postgres_basics::sql_state::SqlState;
use postgres_basics::{FnInfo, Location};
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};
