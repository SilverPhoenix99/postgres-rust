#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum LexerErrorKind {

    #[error("Unexpected character {0:?}", *(.unknown))]
    UnexpectedChar { unknown: char },

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
    InvalidInteger { radix: u32 },

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

impl HasSqlState for LexerErrorKind {
    #[inline(always)]
    fn sql_state(&self) -> SqlState {
        SyntaxError
    }
}

impl ErrorReport for LexerErrorKind {
    #[inline(always)]
    fn detail(&self) -> Option<Cow<'static, str>> {
        if UnsafeUnicodeString.eq(self) {
            Some(
                r#"String constants with Unicode escapes cannot be used when "standard_conforming_strings" is off."#.into()
            )
        }
        else {
            None
        }
    }
}

use postgres_basics::{
    elog::{ErrorReport, HasSqlState},
    sql_state::SqlState,
    sql_state::SqlState::SyntaxError,
};
use std::borrow::Cow;
use LexerErrorKind::UnsafeUnicodeString;
