use crate::lexer::keyword::keywords::KEYWORDS;
use crate::lexer::Keyword;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct KeywordDetails {
    pub keyword: Keyword,
    pub text: &'static [u8],
    pub bare: bool,
}

impl KeywordDetails {

    pub(super) const fn new(keyword: Keyword, text: &'static [u8], bare: bool,) -> Self {
        KeywordDetails { keyword, text, bare }
    }

    #[inline(always)]
    pub fn string(&self) -> String {
        unsafe {
            // Simply use `text`, since it's already lowercase
            String::from_utf8_unchecked(self.text.to_vec())
        }
    }

    #[inline(always)]
    pub fn find(text: &[u8]) -> Option<&'static KeywordDetails> {
        KEYWORDS.get(text)
    }
}
