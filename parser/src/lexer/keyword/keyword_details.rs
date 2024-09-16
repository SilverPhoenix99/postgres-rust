use crate::lexer::keyword::keywords::KEYWORDS;
use crate::lexer::Keyword;
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
}

impl Display for KeywordDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        // All keywords are ascii strings
        let text = std::str::from_utf8(self.text).unwrap();

        f.write_str(text)
    }
}
