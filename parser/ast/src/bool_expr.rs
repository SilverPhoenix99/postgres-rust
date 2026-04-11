#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BoolExpr {
    Not(Box<ExprNode>),
    Or(Vec<ExprNode>),
    And(Vec<ExprNode>)
}

use crate::ExprNode;
