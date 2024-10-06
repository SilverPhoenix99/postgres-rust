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
