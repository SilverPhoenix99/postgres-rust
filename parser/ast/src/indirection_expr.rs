#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndirectionExpr {
    expr: ExprNode,
    indirection: Vec<Indirection>,
}

impl IndirectionExpr {
    pub fn new(expr: ExprNode, indirection: Vec<Indirection>) -> Self {
        debug_assert!(!indirection.is_empty());
        Self { indirection, expr }
    }

    pub fn indirection(&self) -> &[Indirection] {
        &self.indirection
    }

    pub fn expr(&self) -> &ExprNode {
        &self.expr
    }
}

use crate::ExprNode;
use crate::Indirection;
