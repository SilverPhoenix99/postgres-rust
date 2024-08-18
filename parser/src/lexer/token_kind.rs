#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    OpenParenthesis,
    CloseParenthesis,
    Comma,
    Semicolon,
    OpenBracket,
    CloseBracket,
    Dot,
    DotDot,
    Colon,
    Typecast,
    ColonEquals,
    Percent,
    Mul,
    Plus,
    Minus,
    Div,
    Less,
    Equals,
    Greater,
    Circumflex,
    EqualsGreater,
    LessEquals,
    GreaterEquals,
    NotEquals,
    UserDefinedOperator,
    Param { index: i32 },
    Identifier,
    NumberLiteral { radix: i32 },
    StringLiteral {
        /// If this literal can be automatically concatenated
        /// with the previous StringLiteral.
        /// E.g.:
        /// ```sql
        /// SELECT 'this (previous) string'
        ///   -- some comment and whitespace
        ///     ' can be concatenated with this one'
        /// ```
        concatenable: bool
    },
}
