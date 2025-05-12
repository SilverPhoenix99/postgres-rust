#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnaryExpr {
    operator: QualifiedOperator,
    operand: ExprNode,
}

impl UnaryExpr {
    pub fn new(operator: QualifiedOperator, operand: ExprNode) -> Self {
        Self { operator, operand }
    }

    pub fn unary_plus(operand: ExprNode) -> Self {
        Self::new(Operator::Addition.into(), operand)
    }

    /// Aka `unary_minus`
    pub fn negation(operand: ExprNode) -> Self {
        Self::new(Operator::Subtraction.into(), operand)
    }
}

use crate::parser::ast_node::{ExprNode, Operator, QualifiedOperator};
