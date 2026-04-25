#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RowOverlaps {
    left: (ExprNode, ExprNode),
    left_implicit: bool,
    right: (ExprNode, ExprNode),
    right_implicit: bool,
}

impl RowOverlaps {
    pub fn new(
        left: (ExprNode, ExprNode),
        left_implicit: bool,
        right: (ExprNode, ExprNode),
        right_implicit: bool
    ) -> Self {
        Self {
            left,
            left_implicit,
            right,
            right_implicit,
        }
    }

    pub fn left(&self) -> (&ExprNode, &ExprNode) {
        (&self.left.0, &self.left.1)
    }

    pub fn is_left_implicit(&self) -> bool {
        self.left_implicit
    }

    pub fn is_left_explicit(&self) -> bool {
        !self.left_implicit
    }

    pub fn right(&self) -> (&ExprNode, &ExprNode) {
        (&self.right.0, &self.right.1)
    }

    pub fn is_right_implicit(&self) -> bool {
        self.right_implicit
    }

    pub fn is_right_explicit(&self) -> bool {
        !self.right_implicit
    }
}

use crate::ExprNode;
