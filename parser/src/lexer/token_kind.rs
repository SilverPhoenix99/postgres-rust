use crate::lexer::KeywordDetails;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IdentifierKind {

    /// E.g.: `ident`
    BasicIdentifier,

    /// E.g.: `"ident"`
    QuotedIdentifier,

    /// E.g.: `u&"ident"`
    UnicodeIdentifier,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StringKind {

    /// E.g.: `'str'`
    BasicString {
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

    /// E.g.: `e'str'`, `n'str'`, or `'str'`
    /// When `standard_conforming_strings` is `false`:
    ///   * `[eE]` prefix can be omitted;
    ///   * or it can be a National string (`[nN]` prefix).
    ExtendedString {
        /// See `BasicString.concatenable`.
        ///
        /// Only applies when this string doesn't have a prefix
        concatenable: bool
    },

    /// E.g.: `b'010'`
    BinaryString,

    /// E.g.: `x'1af'`
    HexString,

    /// E.g.: `n'str'`
    NationalString,

    /// E.g.: `u&'str'`
    UnicodeString,

    /// E.g.: `$foo$str$foo$`
    DollarString,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    Keyword(&'static KeywordDetails),
    NumberLiteral { radix: i32 },
    StringLiteral(StringKind),
}

impl TokenKind {

    #[inline(always)]
    pub fn identifier_kind(&self) -> Option<IdentifierKind> {
        match self {
            TokenKind::Identifier(kind) => Some(*kind),
            _ => None
        }
    }

    #[inline(always)]
    pub fn string_kind(&self) -> Option<StringKind> {
        match self {
            TokenKind::StringLiteral(kind) => Some(*kind),
            _ => None
        }
    }

    #[inline(always)]
    pub fn unreserved_keyword(&self) -> Option<&'static KeywordDetails> {
        self.keyword().filter(|kw| kw.unreserved().is_some())
    }

    #[inline(always)]
    pub fn col_name_keyword(&self) -> Option<&'static KeywordDetails> {
        self.keyword().filter(|kw| kw.col_name().is_some())
    }

    #[inline(always)]
    pub fn type_func_name_keyword(&self) -> Option<&'static KeywordDetails> {
        self.keyword().filter(|kw| kw.type_func_name().is_some())
    }

    #[inline(always)]
    pub fn reserved_keyword(&self) -> Option<&'static KeywordDetails> {
        self.keyword().filter(|kw| kw.reserved().is_some())
    }

    #[inline(always)]
    pub fn bare_label_keyword(&self) -> Option<&'static KeywordDetails> {
        self.keyword().filter(|kw| kw.bare())
    }

    #[inline(always)]
    pub fn keyword(&self) -> Option<&'static KeywordDetails> {
        match self {
            TokenKind::Keyword(kw) => Some(kw),
            _ => None
        }
    }
}
