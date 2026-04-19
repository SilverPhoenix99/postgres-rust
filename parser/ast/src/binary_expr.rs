#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinaryExpr {
    operator: QualifiedOperator,
    left_operand: ExprNode,
    right_operand: ExprNode,
}

impl BinaryExpr {
    pub fn new<T, L, R>(operator: T, left_operand: L, right_operand: R) -> Self
    where
        T: Into<QualifiedOperator>,
        L: Into<ExprNode>,
        R: Into<ExprNode>,
    {
        Self {
            operator: operator.into(),
            left_operand: left_operand.into(),
            right_operand: right_operand.into(),
        }
    }
}

use crate::ExprNode;
use crate::QualifiedOperator;
