#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonValueExpr {
    expr: ExprNode,
    format: JsonFormat,
}

impl JsonValueExpr {

    pub fn new(expr: ExprNode) -> Self {
        Self {
            expr,
            format: Default::default()
        }
    }

    pub fn expr(&self) -> &ExprNode {
        &self.expr
    }

    pub fn set_format(&mut self, format: JsonFormat) -> &mut Self {
        self.format = format;
        self
    }
    
    pub fn with_format(mut self, format: JsonFormat) -> Self {
        self.format = format;
        self
    }

    pub fn format(&self) -> JsonFormat {
        self.format
    }
}

impl From<ExprNode> for JsonValueExpr {
    fn from(expr: ExprNode) -> Self {
        Self::new(expr)
    }
}

use crate::ExprNode;
use crate::JsonFormat;
