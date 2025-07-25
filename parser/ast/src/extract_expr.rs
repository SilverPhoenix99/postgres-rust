#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractFunc {
    field: ExtractArg,
    target: ExprNode,
}

impl ExtractFunc {
    pub fn new(field: ExtractArg, target: ExprNode) -> Self {
        Self { field, target }
    }

    pub fn field(&self) -> &ExtractArg {
        &self.field
    }

    pub fn target(&self) -> &ExprNode {
        &self.target
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractArg {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Named(Box<str>),
}

use crate::ExprNode;
