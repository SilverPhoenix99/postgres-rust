mod lexer_error_kind;

use crate::lexer::LexerErrorKind::UnsafeUnicodeString;
use crate::parser::ParseReport;
pub use lexer_error_kind::LexerErrorKind;
use postgres_basics::sql_state::ErrorSqlState::SyntaxError;
use postgres_basics::sql_state::SqlState;
use postgres_basics::{FnInfo, Location, SqlReport};
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LexerError {
    source: LexerErrorKind,
    fn_info: FnInfo,
    location: Location,
}

impl LexerError {

    #[inline(always)]
    pub fn new(source: LexerErrorKind, fn_info: FnInfo, location: Location) -> Self {
        Self { source, fn_info, location }
    }

    #[inline(always)]
    pub fn source(&self) -> LexerErrorKind {
        self.source
    }
}

impl Error for LexerError {

    #[inline(always)]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sqlstate= self.sqlstate();
        let source = self.source;
        let position = self.location.range().start + 1;
        writeln!(f, "[{sqlstate}] ERROR: {source}")?;
        write!(f, "Position: {position}")
    }
}

impl ParseReport for LexerError {

    #[inline(always)]
    fn location(&self) -> &Location {
        &self.location
    }
}

impl SqlReport for LexerError {

    fn sqlstate(&self) -> SqlState {
        SqlState::Error(SyntaxError)
    }

    #[inline(always)]
    fn fn_info(&self) -> &FnInfo {
        &self.fn_info
    }

    fn detail(&self) -> Option<Cow<'static, str>> {
        if self.source == UnsafeUnicodeString {
            Some(
                r#"String constants with Unicode escapes cannot be used when "standard_conforming_strings" is off."#.into()
            )
        }
        else {
            None
        }
    }
}
