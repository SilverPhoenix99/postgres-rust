#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrimFunc {
    trim_side: TrimSide,
    args: Vec<ExprNode>,
}

impl TrimFunc {
    pub fn new(trim_side: TrimSide, args: Vec<ExprNode>) -> Self {
        Self { trim_side, args }
    }

    pub fn trim_side(&self) -> TrimSide {
        self.trim_side
    }

    pub fn args(&self) -> &[ExprNode] {
        &self.args
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TrimSide {
    #[default]
    Both,
    Leading,
    Trailing,
}

use crate::ExprNode;
