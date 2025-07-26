#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionFunc {
    needle: ExprNode,
    haystack: ExprNode,
}

impl PositionFunc {
    pub fn new(needle: ExprNode, haystack: ExprNode) -> Self {
        Self { needle, haystack }
    }

    pub fn needle(&self) -> &ExprNode {
        &self.needle
    }

    pub fn haystack(&self) -> &ExprNode {
        &self.haystack
    }
}

use crate::ExprNode;
