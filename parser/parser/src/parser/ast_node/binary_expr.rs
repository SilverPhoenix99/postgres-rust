#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinaryExpr {
    operator: QualifiedOperator,
    left_operand: ExprNode,
    right_operand: ExprNode,
}

impl BinaryExpr {
    pub fn new(operator: QualifiedOperator, left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self { operator, left_operand, right_operand, }
    }

    pub fn addition(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Addition.into(), left_operand, right_operand)
    }

    pub fn subtraction(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Subtraction.into(), left_operand, right_operand)
    }

    pub fn multiplication(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Multiplication.into(), left_operand, right_operand)
    }

    pub fn division(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Division.into(), left_operand, right_operand)
    }

    pub fn modulo(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Modulo.into(), left_operand, right_operand)
    }

    pub fn exponentiation(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Exponentiation.into(), left_operand, right_operand)
    }

    pub fn less(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Less.into(), left_operand, right_operand)
    }

    pub fn greater(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Greater.into(), left_operand, right_operand)
    }

    pub fn equals(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Equals.into(), left_operand, right_operand)
    }

    pub fn less_equals(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::LessEquals.into(), left_operand, right_operand)
    }

    pub fn greater_equals(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::GreaterEquals.into(), left_operand, right_operand)
    }

    pub fn not_equals(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::NotEquals.into(), left_operand, right_operand)
    }
}

use crate::parser::ast_node::{ExprNode, Operator, QualifiedOperator};
