#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ExtendedStringWarning {
    NonstandardEscape,
    NonstandardQuoteEscape,
    NonstandardBackslashEscape,
}

impl ExtendedStringWarning {

    #[inline(always)]
    pub fn sqlstate(self) -> SqlState {
        SqlState::Error(ErrorSqlState::NonstandardUseOfEscapeCharacter)
    }

    #[inline]
    pub fn message(self) -> &'static str {
        match self {
            NonstandardEscape => "nonstandard use of escape in a string literal",
            NonstandardQuoteEscape => r"nonstandard use of \' in a string literal",
            NonstandardBackslashEscape => r"nonstandard use of \\ in a string literal",
        }
    }

    #[inline]
    pub fn hint(self) -> &'static str {
        match self {
            NonstandardEscape => r"Use the escape string syntax for escapes, e.g., E'\r\n'.",
            NonstandardQuoteEscape => "Use '' to write quotes in strings, or use the escape string syntax (E'...').",
            NonstandardBackslashEscape => r"Use the escape string syntax for backslashes, e.g., E'\\'.",
        }
    }
}

use postgres_basics::sql_state::{ErrorSqlState, SqlState};
use ExtendedStringWarning::*;
