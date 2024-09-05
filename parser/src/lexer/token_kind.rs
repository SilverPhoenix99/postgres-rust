
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IdentifierKind {

    /// E.g.: `ident`
    BasicIdentifier,

    /// E.g.: `"ident"`
    QuotedIdentifier,

    /// E.g.: `u&"ident"`
    UnicodeIdentifier,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StringKind {

    /// E.g.: `'str'`
    BasicString,

    /// E.g.: `e'str'`
    /// When `standard_conforming_strings` is `false`:
    ///   * `[eE]` prefix can be omitted;
    ///   * or it can be a National string (`[nN]` prefix).
    ExtendedString,

    /// E.g.: `b'010'` or `x'1af'`
    BitString,

    /// E.g.: `n'str'`
    NationalString,

    /// E.g.: `u&'str'`
    UnicodeString,

    /// E.g.: `$foo$str$foo$`
    DollarString,
}

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
    Identifier(IdentifierKind),
    NumberLiteral { radix: i32 },
    StringLiteral {
        kind: StringKind,
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
