use std::ops::Range;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    // TODO premature opt:
    //   this data is packed together
    //   considering generics in the future,
    //   where it could be replaced with ()
    pub details: (Range<usize>, (usize, usize))
}

impl Token {

    #[inline]
    pub fn new(kind: TokenKind, details: (Range<usize>, (usize, usize))) -> Self {
        Self { kind, details }
    }
}
