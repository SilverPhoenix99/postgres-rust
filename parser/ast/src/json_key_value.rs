#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonKeyValue {
    key: ExprNode,
    value: JsonValueExpr
}

impl JsonKeyValue {
    pub fn new(key: ExprNode, value: JsonValueExpr) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &ExprNode {
        &self.key
    }

    pub fn value(&self) -> &JsonValueExpr {
        &self.value
    }
}

use crate::ExprNode;
use crate::JsonValueExpr;
