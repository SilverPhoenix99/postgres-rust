mod keyword_details;
mod keywords;

pub use keyword_details::KeywordDetails;
pub use keywords::{ColumnNameKeyword, ReservedKeyword, TypeFuncNameKeyword, UnreservedKeyword};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    Unreserved(UnreservedKeyword),
    ColumnName(ColumnNameKeyword),
    TypeFuncName(TypeFuncNameKeyword),
    Reserved(ReservedKeyword),
}
