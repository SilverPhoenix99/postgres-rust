#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CaseExpr {
    target: Option<ExprNode>,
    when_clauses: Vec<CaseWhen>,
    default: Option<ExprNode>,
}

impl CaseExpr {
    pub fn new(target: Option<ExprNode>, when_clauses: Vec<CaseWhen>, default: Option<ExprNode>) -> Self {
        Self { target, when_clauses, default }
    }

    pub fn target(&self) -> &Option<ExprNode> {
        &self.target
    }

    pub fn when_clauses(&self) -> &[CaseWhen] {
        &self.when_clauses
    }

    pub fn default(&self) -> &Option<ExprNode> {
        &self.default
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CaseWhen {
    condition: ExprNode,
    body: ExprNode,
}

impl CaseWhen {
    pub fn new(condition: ExprNode, body: ExprNode) -> Self {
        Self { condition, body }
    }

    pub fn condition(&self) -> &ExprNode {
        &self.condition
    }

    pub fn body(&self) -> &ExprNode {
        &self.body
    }
}

use crate::parser::ast_node::ExprNode;
