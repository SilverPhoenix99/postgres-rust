use crate::lexer::keyword::keywords::KEYWORDS;
use crate::lexer::Keyword;
use crate::lexer::Keyword::{ColumnName, Reserved, TypeFuncName, Unreserved};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct KeywordDetails {
    pub keyword: Keyword,
    pub text: &'static [u8],
    pub bare: bool,
}

impl KeywordDetails {

    #[inline(always)]
    pub(super) const fn new(keyword: Keyword, text: &'static [u8], bare: bool,) -> Self {
        KeywordDetails { keyword, text, bare }
    }

    #[inline(always)]
    pub fn find(text: &[u8]) -> Option<&'static KeywordDetails> {
        KEYWORDS.get(text)
    }

    #[inline(always)]
    pub fn unreserved(&self) -> bool {
        matches!(self.keyword, Unreserved(_))
    }

    #[inline(always)]
    pub fn col_name(&self) -> bool {
        matches!(self.keyword, ColumnName(_))
    }

    #[inline(always)]
    pub fn type_func_name(&self) -> bool {
        matches!(self.keyword, TypeFuncName(_))
    }

    #[inline(always)]
    pub fn reserved(&self) -> bool {
        matches!(self.keyword, Reserved(_))
    }
}

impl Display for KeywordDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        // All keywords are ascii strings
        let text = std::str::from_utf8(self.text).unwrap();

        f.write_str(text)
    }
}
