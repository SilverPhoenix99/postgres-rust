#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinaryExpr {
    operator: QualifiedOperator,
    left_operand: ExprNode,
    right_operand: ExprNode,
}

impl BinaryExpr {
    pub fn new<T>(operator: T, left_operand: ExprNode, right_operand: ExprNode) -> Self
    where
        T: Into<QualifiedOperator>,
    {
        Self {
            operator: operator.into(),
            left_operand,
            right_operand,
        }
    }
}

use crate::ExprNode;
use pg_sink_ast::QualifiedOperator;
