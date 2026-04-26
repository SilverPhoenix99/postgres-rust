#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimezoneExpr {
    expression: ExprNode,

    /// When absent, represents the local time zone.
    zone: Option<ExprNode>
}

impl TimezoneExpr {

    pub fn new(expression: ExprNode, zone: Option<ExprNode>) -> Self {
        Self { expression, zone }
    }

    pub fn expression(&self) -> &ExprNode {
        &self.expression
    }

    pub fn zone(&self) -> Option<&ExprNode> {
        self.zone.as_ref()
    }
}

use crate::ExprNode;
