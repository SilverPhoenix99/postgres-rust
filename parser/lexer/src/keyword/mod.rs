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

    pub fn category(&self) -> KeywordCategory {
        self.details().category()
    }

    pub fn text(&self) -> &'static str {
        self.details().text()
    }
}

impl From<Keyword> for Str {
    fn from(value: Keyword) -> Self {
        value.text().into()
    }
}

use crate::keyword::keywords::ENTRIES;
use crate::keyword::keywords::MAP;
use crate::keyword::keywords::MAX_KEYWORD_LENGTH;
use pg_basics::Str;
use std::fmt::Debug;
