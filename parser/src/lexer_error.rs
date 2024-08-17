use std::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LexerErrorCode {
    Eof,
    UnknownChar { unknown: u8 },
    UnterminatedBlockComment,
    OperatorTooLong,
    ParameterNumberTooLarge,
    TrailingJunkAfterParameter,
    InvalidInteger { radix: i32 },
    TrailingJunkAfterNumericLiteral,
    InvalidTokenRange,
    UnterminatedString,
    InvalidUnicodeEscape,
    EmptyDelimitedIdentifier,
    UnsafeUnicodeString,
    UnterminatedQuotedIdentifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    pub error_code: LexerErrorCode,
    // TODO premature opt:
    //   this data is packed together
    //   considering generics in the future,
    //   where it could be replaced with ()
    pub details: (Range<usize>, (usize, usize))
}

impl LexerError {

    #[inline]
    pub fn new(error_code: LexerErrorCode, details: (Range<usize>, (usize, usize))) -> Self {
        Self { error_code, details }
    }
}
