mod keyword_details;
mod keywords;

pub use self::{
    keyword_details::KeywordDetails,
    keywords::{
        ColumnNameKeyword,
        ReservedKeyword,
        TypeFuncNameKeyword,
        UnreservedKeyword
    },
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Keyword {
    Unreserved(UnreservedKeyword),
    ColumnName(ColumnNameKeyword),
    TypeFuncName(TypeFuncNameKeyword),
    Reserved(ReservedKeyword),
}
