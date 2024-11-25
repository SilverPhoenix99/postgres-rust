mod keyword_details;
mod keywords;

pub use self::keyword_details::KeywordDetails;
pub use self::keywords::Keyword;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum KeywordCategory {
    Unreserved,
    ColumnName,
    TypeFuncName,
    Reserved,
}

impl Keyword {
    pub fn find(text: &str) -> Option<Self> {

        if text.is_empty() || text.len() > MAX_KEYWORD_LENGTH || !text.is_ascii() {
            return None
        }

        let text = text.to_ascii_lowercase();

        MAP.get(&text)
            .map(KeywordDetails::keyword)
    }

    pub fn details(&self) -> &'static KeywordDetails {
        &ENTRIES[*self as usize].1
    }
}

impl From<Keyword> for Str {
    fn from(value: Keyword) -> Self {
        value.details().text().into()
    }
}

use crate::lexer::keyword::keywords::ENTRIES;
use crate::lexer::keyword::keywords::MAP;
use crate::lexer::keyword::keywords::MAX_KEYWORD_LENGTH;
use postgres_basics::Str;
use std::fmt::Debug;
