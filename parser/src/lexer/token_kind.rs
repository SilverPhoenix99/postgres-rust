#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum IdentifierKind {

    /// E.g.: `ident`
    Basic,

    /// E.g.: `"ident"`
    Quoted,

    /// E.g.: `u&"ident"`
    Unicode,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum StringKind {

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
pub(crate) enum BitStringKind {
    /// E.g.: `b'010'`
    Binary,

    /// E.g.: `x'1af'`
    Hex,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum OperatorKind {
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
pub(crate) enum RawTokenKind {
    Operator(OperatorKind),
    UserDefinedOperator,
    Param { index: i32 },
    Identifier(IdentifierKind),
    Keyword(Keyword),
    NumberLiteral(NumberRadix),
    StringLiteral(StringKind),
    BitStringLiteral(BitStringKind),
}

impl RawTokenKind {

    #[inline(always)]
    pub fn operator(&self) -> Option<OperatorKind> {
        match self {
            RawTokenKind::Operator(kind) => Some(*kind),
            _ => None
        }
    }

    #[inline(always)]
    pub fn identifier(&self) -> Option<IdentifierKind> {
        match self {
            RawTokenKind::Identifier(kind) => Some(*kind),
            _ => None
        }
    }

    #[inline(always)]
    pub fn string(&self) -> Option<StringKind> {
        match self {
            RawTokenKind::StringLiteral(kind) => Some(*kind),
            _ => None
        }
    }

    #[inline(always)]
    pub fn bit_string(&self) -> Option<BitStringKind> {
        match self {
            RawTokenKind::BitStringLiteral(kind) => Some(*kind),
            _ => None
        }
    }

    #[inline(always)]
    pub fn unreserved_keyword(&self) -> Option<Keyword> {
        self.keyword().and_then(|kw| kw.details().unreserved())
    }

    #[inline(always)]
    pub fn col_name_keyword(&self) -> Option<Keyword> {
        self.keyword().and_then(|kw| kw.details().col_name())
    }

    #[inline(always)]
    pub fn type_func_name_keyword(&self) -> Option<Keyword> {
        self.keyword().and_then(|kw| kw.details().type_func_name())
    }

    #[inline(always)]
    pub fn reserved_keyword(&self) -> Option<Keyword> {
        self.keyword().and_then(|kw| kw.details().reserved())
    }

    #[inline(always)]
    pub fn bare_label_keyword(&self) -> Option<Keyword> {
        self.keyword().filter(|kw| kw.details().bare())
    }

    #[inline(always)]
    pub fn keyword(&self) -> Option<Keyword> {
        match self {
            RawTokenKind::Keyword(kw) => Some(*kw),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BitStringKind::*;
    use OperatorKind::{CloseParenthesis, Semicolon};
    use RawTokenKind::*;

    #[test]
    fn test_operator_kind() {
        assert_eq!(None, Identifier(IdentifierKind::Basic).operator());
        assert_eq!(Some(CloseParenthesis), Operator(CloseParenthesis).operator())
    }

    #[test]
    fn test_identifier_kind() {
        assert_eq!(Some(IdentifierKind::Basic), Identifier(IdentifierKind::Basic).identifier());
        assert_eq!(None, Operator(CloseParenthesis).identifier())
    }

    #[test]
    fn test_string_kind() {
        assert_eq!(Some(StringKind::Unicode), StringLiteral(StringKind::Unicode).string());
        assert_eq!(None, Operator(CloseParenthesis).string())
    }

    #[test]
    fn test_bit_string_kind() {
        assert_eq!(Some(Hex), BitStringLiteral(Hex).bit_string());
        assert_eq!(None, Operator(CloseParenthesis).bit_string())
    }

    #[test]
    fn test_unreserved_keyword() {
        let kw = super::Keyword::find("abort").unwrap();
        assert_eq!(Some(kw), Keyword(kw).unreserved_keyword());

        let kw = super::Keyword::find("between").unwrap();
        assert_eq!(None, Keyword(kw).unreserved_keyword())
    }

    #[test]
    fn test_col_name_keyword() {
        let kw = super::Keyword::find("between").unwrap();
        assert_eq!(Some(kw), Keyword(kw).col_name_keyword());

        let kw = super::Keyword::find("authorization").unwrap();
        assert_eq!(None, Keyword(kw).col_name_keyword())
    }

    #[test]
    fn test_type_func_name_keyword() {
        let kw = super::Keyword::find("authorization").unwrap();
        assert_eq!(Some(kw), Keyword(kw).type_func_name_keyword());

        let kw = super::Keyword::find("analyze").unwrap();
        assert_eq!(None, Keyword(kw).type_func_name_keyword())
    }

    #[test]
    fn test_reserved_keyword() {
        let kw = super::Keyword::find("analyze").unwrap();
        assert_eq!(Some(kw), Keyword(kw).reserved_keyword());

        let kw = super::Keyword::find("abort").unwrap();
        assert_eq!(None, Keyword(kw).reserved_keyword())
    }

    #[test]
    fn test_bare_label_keyword() {
        let kw = super::Keyword::find("abort").unwrap();
        assert_eq!(Some(kw), Keyword(kw).bare_label_keyword());

        let kw = super::Keyword::find("array").unwrap();
        assert_eq!(None, Keyword(kw).bare_label_keyword())
    }

    #[test]
    fn test_keyword() {
        let kw = super::Keyword::find("between").unwrap();
        assert_eq!(Some(kw), Keyword(kw).keyword());

        assert_eq!(None, Operator(Semicolon).keyword());
    }
}

use crate::lexer::Keyword;
use crate::NumberRadix;
