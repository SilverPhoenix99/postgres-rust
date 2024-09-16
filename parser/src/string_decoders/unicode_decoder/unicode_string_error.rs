use std::str::Utf8Error;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UnicodeStringError {
    Utf8(Utf8Error),
    InvalidUnicodeCodepoint,
    InvalidUnicodeSurrogatePair,
}
