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

impl From<ExprNode> for JsonValueExpr {
    fn from(expr: ExprNode) -> Self {
        Self::new(expr, JsonFormat::default())
    }
}

use crate::ExprNode;
use crate::JsonFormat;
