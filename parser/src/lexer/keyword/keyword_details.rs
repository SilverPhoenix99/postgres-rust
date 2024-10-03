#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KeywordDetails {
    keyword: Keyword,
    text: &'static str,
    bare: bool,
}

impl KeywordDetails {

    #[inline(always)]
    pub(super) const fn new(keyword: Keyword, text: &'static str, bare: bool) -> Self {
        KeywordDetails { keyword, text, bare }
    }

    #[inline(always)]
    pub fn find(text: &str) -> Option<&'static KeywordDetails> {
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
        self.text
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
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unreserved() {
        let kw = KEYWORDS.get("abort").unwrap();
        assert_eq!(Some(UnreservedKeyword::Abort), kw.unreserved());

        let kw = KEYWORDS.get("between").unwrap();
        assert_eq!(None, kw.unreserved());
    }

    #[test]
    fn test_col_name() {
        let kw = KEYWORDS.get("between").unwrap();
        assert_eq!(Some(ColumnNameKeyword::Between), kw.col_name());

        let kw = KEYWORDS.get("authorization").unwrap();
        assert_eq!(None, kw.col_name());
    }

    #[test]
    fn test_type_func_name() {
        let kw = KEYWORDS.get("authorization").unwrap();
        assert_eq!(Some(TypeFuncNameKeyword::Authorization), kw.type_func_name());

        let kw = KEYWORDS.get("analyze").unwrap();
        assert_eq!(None, kw.type_func_name());
    }

    #[test]
    fn test_reserved() {
        let kw = KEYWORDS.get("analyze").unwrap();
        assert_eq!(Some(ReservedKeyword::Analyze), kw.reserved());

        let kw = KEYWORDS.get("abort").unwrap();
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
