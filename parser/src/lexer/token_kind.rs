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

    /// E.g.: `n'str'`
    National,

    /// E.g.: `u&'str'`
    Unicode,

    /// E.g.: `$foo$str$foo$`
    Dollar,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BitStringKind {
    /// E.g.: `b'010'`
    Binary,

    /// E.g.: `x'1af'`
    Hex,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenKind {
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
    UserDefinedOperator,
    Param { index: i32 },
    Identifier(IdentifierKind),
    Keyword(Keyword),
    NumberLiteral { radix: u32 },
    StringLiteral(StringKind),
    BitStringLiteral(BitStringKind),
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
    pub fn bit_string_kind(&self) -> Option<BitStringKind> {
        match self {
            TokenKind::BitStringLiteral(kind) => Some(*kind),
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
            TokenKind::Keyword(kw) => Some(*kw),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BitStringKind::*;
    use TokenKind::*;

    #[test]
    fn test_identifier_kind() {
        assert_eq!(Some(IdentifierKind::Basic), Identifier(IdentifierKind::Basic).identifier_kind());
        assert_eq!(None, CloseParenthesis.identifier_kind())
    }

    #[test]
    fn test_string_kind() {
        assert_eq!(Some(StringKind::Unicode), StringLiteral(StringKind::Unicode).string_kind());
        assert_eq!(None, CloseParenthesis.string_kind())
    }

    #[test]
    fn test_bit_string_kind() {
        assert_eq!(Some(Hex), BitStringLiteral(Hex).bit_string_kind());
        assert_eq!(None, CloseParenthesis.bit_string_kind())
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

        assert_eq!(None, Semicolon.keyword());
    }
}

use crate::lexer::Keyword;
