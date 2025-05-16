#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IdentifierKind {

    /// E.g.: `ident`
    Basic,

    /// E.g.: `"ident"`
    Quoted,

    /// E.g.: `u&"ident"`
    Unicode,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StringKind {

    /// E.g.: `'str'`
    Basic {
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
    Extended {
        /// See `BasicString.concatenable`.
        ///
        /// Only applies when this string doesn't have a prefix
        concatenable: bool
    },

    /// E.g.: `u&'str'`
    Unicode,

    /// E.g.: `$foo$str$foo$`
    Dollar,
}

impl StringKind {
    pub fn is_concatenable(self) -> bool {
        match self {
            Self::Basic { concatenable } | Self::Extended { concatenable } => concatenable,
            Self::Unicode | Self::Dollar => false
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BitStringKind {
    /// E.g.: `b'010'`
    Binary,

    /// E.g.: `x'1af'`
    Hex,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperatorKind {
    /// `(`
    OpenParenthesis,
    /// `)`
    CloseParenthesis,
    /// `,`
    Comma,
    /// `;`
    Semicolon,
    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `:`
    Colon,
    /// The cast operator `::`
    Typecast,
    /// `:=`
    ColonEquals,
    /// `%`
    Percent,
    /// `*`, aka `Star`
    Mul,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `/`
    Div,
    /// `<`
    Less,
    /// `=`
    Equals,
    /// `>`
    Greater,
    /// `^`
    Circumflex,
    /// `=>`
    EqualsGreater,
    /// `<=`
    LessEquals,
    /// `>=`
    GreaterEquals,
    /// `<>` or `!=`
    NotEquals,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RawTokenKind {
    Param { index: i32 },
    Keyword(Keyword),
    Operator(OperatorKind),
    UserDefinedOperator,
    Identifier(IdentifierKind),
    NumberLiteral(NumberRadix),
    StringLiteral(StringKind),
    BitStringLiteral(BitStringKind),
}

use crate::Keyword;
use pg_basics::NumberRadix;
