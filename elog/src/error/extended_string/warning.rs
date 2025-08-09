#[derive(Debug, Copy, Clone, Eq, PartialEq, Display)]
pub enum Warning {

    #[display("nonstandard use of escape in a string literal")]
    NonstandardEscape,

    #[display(r"nonstandard use of \' in a string literal")]
    NonstandardQuoteEscape,

    #[display(r"nonstandard use of \\ in a string literal")]
    NonstandardBackslashEscape,
}

impl core::error::Error for Warning {}

impl LogMessage for Warning {

    fn sql_state(&self) -> SqlState {
        SqlState::NonstandardUseOfEscapeCharacter
    }

    fn hint(&self) -> Option<&str> {
        match self {
            Self::NonstandardEscape => Some(r"Use the escape string syntax for escapes, e.g., E'\r\n'."),
            Self::NonstandardQuoteEscape => {
                Some("Use '' to write quotes in strings, or use the escape string syntax (E'...').")
            },
            Self::NonstandardBackslashEscape => {
                Some(r"Use the escape string syntax for backslashes, e.g., E'\\'.")
            },
        }
    }
}

use crate::sql_state::SqlState;
use crate::LogMessage;
use derive_more::Display;
