#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KeywordDetails {
    keyword: Keyword,
    text: &'static str,
    category: KeywordCategory,
    bare: bool,
}

impl KeywordDetails {

    #[inline(always)]
    pub(super) const fn new(keyword: Keyword, text: &'static str, category: KeywordCategory, bare: bool) -> Self {
        KeywordDetails { keyword, text, category, bare }
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
    pub fn unreserved(&self) -> Option<Keyword> {
        if self.category == Unreserved {
            return Some(self.keyword)
        }
        None
    }

    #[inline(always)]
    pub fn col_name(&self) -> Option<Keyword> {
        if self.category == ColumnName {
            return Some(self.keyword)
        }
        None
    }

    #[inline(always)]
    pub fn type_func_name(&self) -> Option<Keyword> {
        if self.category == TypeFuncName {
            return Some(self.keyword)
        }
        None
    }

    #[inline(always)]
    pub fn reserved(&self) -> Option<Keyword> {
        if self.category == Reserved {
            return Some(self.keyword)
        }
        None
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
    use crate::lexer::Keyword::{Abort, Analyze, Authorization, Between};

    #[test]
    fn test_unreserved() {
        let kw = KEYWORDS.get("abort").unwrap();
        assert_eq!(Some(Abort), kw.unreserved());

        let kw = KEYWORDS.get("between").unwrap();
        assert_eq!(None, kw.unreserved());
    }

    #[test]
    fn test_col_name() {
        let kw = KEYWORDS.get("between").unwrap();
        assert_eq!(Some(Between), kw.col_name());

        let kw = KEYWORDS.get("authorization").unwrap();
        assert_eq!(None, kw.col_name());
    }

    #[test]
    fn test_type_func_name() {
        let kw = KEYWORDS.get("authorization").unwrap();
        assert_eq!(Some(Authorization), kw.type_func_name());

        let kw = KEYWORDS.get("analyze").unwrap();
        assert_eq!(None, kw.type_func_name());
    }

    #[test]
    fn test_reserved() {
        let kw = KEYWORDS.get("analyze").unwrap();
        assert_eq!(Some(Analyze), kw.reserved());

        let kw = KEYWORDS.get("abort").unwrap();
        assert_eq!(None, kw.reserved());
    }
}

use super::{
    keywords::KEYWORDS,
    Keyword,
    KeywordCategory,
    KeywordCategory::{ColumnName, Reserved, TypeFuncName, Unreserved},
};
use std::fmt::{Display, Formatter};
