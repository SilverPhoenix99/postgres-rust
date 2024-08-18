#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LexerError {
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
