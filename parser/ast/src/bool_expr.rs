#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BoolExpr {
    Not(Box<ExprNode>),
    Or(BinaryOperands),
    And(BinaryOperands)
}

impl BoolExpr {
    pub fn not(expr: ExprNode) -> Self {
        Self::Not(Box::new(expr))
    }

    pub fn or(left: ExprNode, right: ExprNode) -> Self {
        Self::Or(Box::new((left, right)))
    }

    pub fn and(left: ExprNode, right: ExprNode) -> Self {
        Self::And(Box::new((left, right)))
    }
}

use crate::BinaryOperands;
use crate::ExprNode;
