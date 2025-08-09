pub type LocatedError = Located<Error>;
pub type Result<T> = core::result::Result<T, Error>;
pub type LocatedResult<T> = core::result::Result<T, LocatedError>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Display)]
pub enum Error {

    #[display("Unexpected character {unknown:?}")]
    UnexpectedChar { unknown: char },

    #[display("unterminated /* comment")]
    UnterminatedBlockComment,

    #[display("operator too long")]
    OperatorTooLong,

    #[display("parameter number too large")]
    ParameterNumberTooLarge,

    #[display("trailing junk after parameter")]
    TrailingJunkAfterParameter,

    #[display("invalid {} integer",
        match _0 {
            NumberRadix::Binary => "binary",
            NumberRadix::Octal => "octal",
            _ => "hexadecimal",
        }
    )]
    InvalidInteger(NumberRadix),

    #[display("trailing junk after numeric literal")]
    TrailingJunkAfterNumericLiteral,

    #[display("unterminated bit string literal")]
    UnterminatedBitString,

    #[display("unterminated hexadecimal string literal")]
    UnterminatedHexString,

    #[display("unterminated quoted string")]
    UnterminatedQuotedString,

    #[display("unterminated dollar-quoted string")]
    UnterminatedDollarQuotedString,

    #[display("zero-length delimited identifier")]
    EmptyDelimitedIdentifier,

    #[display("unterminated quoted identifier")]
    UnterminatedQuotedIdentifier,

    #[display("unsafe use of string constant with Unicode escapes")]
    UnsafeUnicodeString,
}

impl core::error::Error for Error {}

impl LogMessage for Error {

    fn sql_state(&self) -> SqlState {
        SyntaxError
    }

    fn detail(&self) -> Option<&str> {
        if UnsafeUnicodeString.eq(self) {
            Some(
                r#"String constants with Unicode escapes cannot be used when "standard_conforming_strings" is off."#
            )
        }
        else {
            None
        }
    }
}

use crate::LogMessage;
use crate::SqlState;
use crate::SqlState::SyntaxError;
use derive_more::Display;
use pg_basics::Located;
use pg_basics::NumberRadix;
use Error::UnsafeUnicodeString;
