mod keyword_details;
mod keywords;

pub use self::keyword_details::KeywordDetails;
pub use self::keywords::Keyword;
use unicase::UniCase;
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum KeywordCategory {
    Unreserved,
    ColumnName,
    TypeFuncName,
    Reserved,
}

impl Keyword {
    pub fn find(text: &str) -> Option<Self> {
        KEYWORDS.get(&UniCase::new(text)).copied()
    }

    pub fn details(&self) -> &'static KeywordDetails {
        &KEYWORD_DETAILS[*self as usize]
    }
}

use crate::lexer::keyword::keywords::{KEYWORDS, KEYWORD_DETAILS};
