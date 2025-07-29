#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonValueExpr {
    expr: ExprNode,
    format: JsonFormat,
}

impl JsonValueExpr {
    pub fn new(expr: ExprNode, format: JsonFormat) -> Self {
        Self { expr, format }
    }

    pub fn expr(&self) -> &ExprNode {
        &self.expr
    }

    pub fn format(&self) -> JsonFormat {
        self.format
    }
}

use crate::ExprNode;
use crate::JsonFormat;
