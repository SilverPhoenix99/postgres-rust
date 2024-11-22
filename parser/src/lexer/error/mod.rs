mod lexer_error_kind;

pub use lexer_error_kind::LexerErrorKind;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LexerError (
    LocatedErrorReport<LexerErrorKind>
);

impl LexerError {

    #[inline(always)]
    pub fn new(source: LexerErrorKind, location: Location) -> Self {
        let report = LocatedErrorReport::new(source, location);
        Self(report)
    }

    #[inline(always)]
    pub fn source(&self) -> LexerErrorKind {
        *self.0.source()
    }
}

impl HasSqlState for LexerError {
    #[inline(always)]
    fn sql_state(&self) -> SqlState {
        self.source().sql_state()
    }
}

impl HasLocation for LexerError {
    #[inline(always)]
    fn location(&self) -> &Location {
        self.0.location()
    }
}

impl ErrorReport for LexerError {
    #[inline(always)]
    fn hint(&self) -> Option<Cow<'static, str>> {
        self.0.source().hint()
    }

    #[inline(always)]
    fn detail(&self) -> Option<Cow<'static, str>> {
        self.0.source().detail()
    }

    #[inline(always)]
    fn detail_log(&self) -> Option<Cow<'static, str>> {
        self.0.source().detail_log()
    }
}

use crate::error::{HasLocation, LocatedErrorReport};
use postgres_basics::elog::ErrorReport;
use postgres_basics::elog::HasSqlState;
use postgres_basics::sql_state::SqlState;
use postgres_basics::Location;
use std::borrow::Cow;
