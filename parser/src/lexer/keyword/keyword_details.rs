#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KeywordDetails {
    keyword: Keyword,
    text: &'static [u8],
    bare: bool,
}

impl KeywordDetails {

    #[inline(always)]
    pub(super) const fn new(keyword: Keyword, text: &'static [u8], bare: bool) -> Self {
        KeywordDetails { keyword, text, bare }
    }

    #[inline(always)]
    pub fn find(text: &[u8]) -> Option<&'static KeywordDetails> {
        KEYWORDS.get(text)
    }

    #[inline(always)]
    pub fn bare(&self) -> bool {
        self.bare
    }

    #[inline(always)]
    pub fn keyword(&self) -> Keyword {
        self.keyword
    }

    #[inline(always)]
    pub fn text(&self) -> &'static str {
        // SAFETY: all keywords are ascii
        unsafe { std::str::from_utf8_unchecked(self.text) }
    }

    #[inline(always)]
    pub fn unreserved(&self) -> Option<UnreservedKeyword> {
        match self.keyword {
            Unreserved(kw) => Some(kw),
            _ => None
        }
    }

    #[inline(always)]
    pub fn col_name(&self) -> Option<ColumnNameKeyword> {
        match self.keyword {
            ColumnName(kw) => Some(kw),
            _ => None
        }
    }

    #[inline(always)]
    pub fn type_func_name(&self) -> Option<TypeFuncNameKeyword> {
        match self.keyword {
            TypeFuncName(kw) => Some(kw),
            _ => None
        }
    }

    #[inline(always)]
    pub fn reserved(&self) -> Option<ReservedKeyword> {
        match self.keyword {
            Reserved(kw) => Some(kw),
            _ => None
        }
    }
}

impl Display for KeywordDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        // All keywords are ascii strings
        let text = std::str::from_utf8(self.text).unwrap();

        f.write_str(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unreserved() {
        let kw = KEYWORDS.get(b"abort").unwrap();
        assert_eq!(Some(UnreservedKeyword::Abort), kw.unreserved());

        let kw = KEYWORDS.get(b"between").unwrap();
        assert_eq!(None, kw.unreserved());
    }

    #[test]
    fn test_col_name() {
        let kw = KEYWORDS.get(b"between").unwrap();
        assert_eq!(Some(ColumnNameKeyword::Between), kw.col_name());

        let kw = KEYWORDS.get(b"authorization").unwrap();
        assert_eq!(None, kw.col_name());
    }

    #[test]
    fn test_type_func_name() {
        let kw = KEYWORDS.get(b"authorization").unwrap();
        assert_eq!(Some(TypeFuncNameKeyword::Authorization), kw.type_func_name());

        let kw = KEYWORDS.get(b"analyze").unwrap();
        assert_eq!(None, kw.type_func_name());
    }

    #[test]
    fn test_reserved() {
        let kw = KEYWORDS.get(b"analyze").unwrap();
        assert_eq!(Some(ReservedKeyword::Analyze), kw.reserved());

        let kw = KEYWORDS.get(b"abort").unwrap();
        assert_eq!(None, kw.reserved());
    }
}

use crate::lexer::{
    keyword::keywords::KEYWORDS,
    ColumnNameKeyword,
    Keyword,
    Keyword::{ColumnName, Reserved, TypeFuncName, Unreserved},
    ReservedKeyword,
    TypeFuncNameKeyword,
    UnreservedKeyword,
};
use std::fmt::{Display, Formatter};
