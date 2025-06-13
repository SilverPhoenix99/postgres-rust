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
    pub fn category(&self) -> KeywordCategory {
        self.category
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.text)
    }
}

#[cfg(test)]
mod tests {
    use crate::keyword::Keyword::{Abort, Analyze, Authorization, Between};

    #[test]
    fn test_unreserved() {
        assert_eq!(Some(Abort), Abort.details().unreserved());
        assert_eq!(None, Between.details().unreserved());
    }

    #[test]
    fn test_col_name() {
        assert_eq!(Some(Between), Between.details().col_name());
        assert_eq!(None, Authorization.details().col_name());
    }

    #[test]
    fn test_type_func_name() {
        assert_eq!(Some(Authorization), Authorization.details().type_func_name());
        assert_eq!(None, Analyze.details().type_func_name());
    }

    #[test]
    fn test_reserved() {
        assert_eq!(Some(Analyze), Analyze.details().reserved());
        assert_eq!(None, Abort.details().reserved());
    }
}

use super::Keyword;
use super::KeywordCategory;
use super::KeywordCategory::ColumnName;
use super::KeywordCategory::Reserved;
use super::KeywordCategory::TypeFuncName;
use super::KeywordCategory::Unreserved;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
