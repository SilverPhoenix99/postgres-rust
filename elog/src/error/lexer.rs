pub type Result<T> = core::result::Result<T, Error>;
pub type LocatedError = LocatedMessage<Error>;
pub type LocatedResult<T> = core::result::Result<T, LocatedError>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Error {

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

    #[error("invalid {} integer",
        match .0 {
            NumberRadix::Binary => "binary",
            NumberRadix::Octal => "octal",
            _ => "hexadecimal",
        }
    )]
    InvalidInteger(NumberRadix),

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

impl LogMessage for Error {

    #[inline(always)]
    fn sql_state(&self) -> SqlState {
        SyntaxError
    }

    #[inline(always)]
    fn detail(&self) -> Option<Str> {
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

use self::Error::UnsafeUnicodeString;
use crate::LocatedMessage;
use crate::LogMessage;
use crate::SqlState;
use crate::SqlState::SyntaxError;
use pg_basics::NumberRadix;
use pg_basics::Str;
