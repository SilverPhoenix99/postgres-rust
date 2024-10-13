mod keyword_details;
mod keywords;

pub use self::keyword_details::KeywordDetails;
pub use self::keywords::Keyword;

const FNV_PRIME: u64 = 0x0100_0000_01b3;

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

        let index = fnv_hash(FNV_PRIME, text.as_bytes());
        let slot = INTERMEDIATE[index];
        if slot == 0 {
            return None
        }

        let index = if slot < 0 {
            // Negative slots are directly indexed.
            // 1 was subtracted to ensure it wasn't confused with slot 0,
            // so now it needs to subtract again.
            (-slot - 1) as usize
        }
        else {
            // 1 was added to ensure it wasn't confused with slot 0,
            // so now it needs to subtract again.
            fnv_hash((slot - 1) as u64, text.as_bytes())
        };

        let kw = KEYWORD_DETAILS[index];

        if text.len() != kw.text().len() {
            return None
        }

        let left = text.as_bytes().iter().copied();
        let right = kw.text().as_bytes().iter().copied();
        let mut iter = left.zip(right);
        if iter.all(|(l, r)| l.to_ascii_lowercase() == r) {
            return Some(kw.keyword());
        }

        None

        // KEYWORDS.get(&UniCase::new(text)).copied()
    }

    pub fn details(&self) -> &'static KeywordDetails {
        &KEYWORD_DETAILS[*self as usize]
    }
}

/// FNV-1a
fn fnv_hash(d: u64, key: &[u8]) -> usize {
    let hash = key.iter()
        .fold(d, |memo, b| {
            let b = b.to_ascii_lowercase() as u64;
            // SAFETY: overflowing bits are supposed to be discarded for this hash function
            (memo ^ b).wrapping_mul(FNV_PRIME)
        });
    (hash % KEYWORD_DETAILS.len() as u64) as usize
}

use crate::lexer::keyword::keywords::{INTERMEDIATE, KEYWORD_DETAILS, MAX_KEYWORD_LENGTH};
