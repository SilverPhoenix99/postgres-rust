#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum LexerError {

    #[error("Unexpected character {0:?}", *(.unknown) as char)]
    UnexpectedChar { unknown: u8 },

    #[error("unterminated /* comment")]
    UnterminatedBlockComment,

    #[error("operator too long")]
    OperatorTooLong,

    #[error("parameter number too large")]
    ParameterNumberTooLarge,

    #[error("trailing junk after parameter")]
    TrailingJunkAfterParameter,

    #[error("invalid {0} integer",
        match .radix {
            2 => "binary",
            8 => "octal",
            _ => "hexadecimal",
        }
    )]
    InvalidInteger { radix: i32 },

    #[error("trailing junk after numeric literal")]
    TrailingJunkAfterNumericLiteral,

    #[error("unterminated bit string literal")]
    UnterminatedBitString,

    #[error("unterminated hexadecimal string literal")]
    UnterminatedHexString,

    #[error("unterminated quoted string")]
    UnterminatedQuotedString,

    #[error("unterminated dollar-quoted string")]
    UnterminatedDollarQuotedString,

    #[error("zero-length delimited identifier")]
    EmptyDelimitedIdentifier,

    #[error("unterminated quoted identifier")]
    UnterminatedQuotedIdentifier,

    #[error("unsafe use of string constant with Unicode escapes")]
    UnsafeUnicodeString,
}

impl LexerError {
    pub fn sqlstate(self) -> SqlState {
        SqlState::Error(ErrorSqlState::SyntaxError)
    }

    pub fn detail(self) -> Option<&'static str> {
        if self == Self::UnsafeUnicodeString {
            return Some(r#"String constants with Unicode escapes cannot be used when "standard_conforming_strings" is off."#)
        }
        None
    }
}

use postgres_basics::sql_state::{ErrorSqlState, SqlState};
