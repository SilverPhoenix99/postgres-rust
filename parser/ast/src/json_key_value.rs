#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonKeyValue {
    key: ExprNode,
    value: JsonValueExpr
}

impl JsonKeyValue {
    pub fn new<T: Into<JsonValueExpr>>(key: ExprNode, value: T) -> Self {
        Self {
            key,
            value: value.into()
        }
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
