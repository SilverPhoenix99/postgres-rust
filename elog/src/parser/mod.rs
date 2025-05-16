mod parser_error_kind;
mod warning;

pub use self::{
    parser_error_kind::{NameList, ParserErrorKind},
    warning::ParserWarningKind,
};

pub type ParserError = LocatedErrorReport<ParserErrorKind>;

use crate::LocatedErrorReport;
