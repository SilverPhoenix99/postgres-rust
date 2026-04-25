#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RowOverlaps {
    left: (ExprNode, ExprNode),
    right: (ExprNode, ExprNode),
}

impl RowOverlaps {
    pub fn new(left: (ExprNode, ExprNode), right: (ExprNode, ExprNode)) -> Self {
        Self { left, right }
    }

    pub fn left(&self) -> (&ExprNode, &ExprNode) {
        (&self.left.0, &self.left.1)
    }

    pub fn right(&self) -> (&ExprNode, &ExprNode) {
        (&self.right.0, &self.right.1)
    }
}

use crate::ExprNode;
