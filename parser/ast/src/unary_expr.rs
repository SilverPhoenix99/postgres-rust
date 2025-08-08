#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnaryExpr {
    operator: QualifiedOperator,
    operand: ExprNode,
}

impl UnaryExpr {
    pub fn new<T>(operator: T, operand: ExprNode) -> Self
    where
        T: Into<QualifiedOperator>,
    {
        Self {
            operator: operator.into(),
            operand
        }
    }
}

use crate::ExprNode;
use pg_sink_ast::QualifiedOperator;
