#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnaryExpr {
    operator: QualifiedOperator,
    operand: ExprNode,
}

impl UnaryExpr {
    pub fn new<T, R>(operator: T, operand: R) -> Self
    where
        T: Into<QualifiedOperator>,
        R: Into<ExprNode>,
    {
        Self {
            operator: operator.into(),
            operand: operand.into(),
        }
    }
}

use crate::ExprNode;
use crate::QualifiedOperator;
