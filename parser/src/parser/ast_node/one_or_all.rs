#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OneOrAll {
    All,
    Name(CowStr),
}

use crate::parser::CowStr;
