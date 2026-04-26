#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CollationExpr {
    expression: ExprNode,
    collation: QualifiedName
}

impl CollationExpr {

    pub fn new(expression: ExprNode, collation: QualifiedName) -> Self {
        Self { expression, collation }
    }

    pub fn expression(&self) -> &ExprNode {
        &self.expression
    }

    pub fn collation(&self) -> &QualifiedName {
        &self.collation
    }
}

use crate::ExprNode;
use pg_basics::QualifiedName;
