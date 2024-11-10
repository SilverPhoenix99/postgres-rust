#[derive(Debug, Clone, PartialEq)]
pub struct TypecastExpr {
    type_name: SystemType,
    value: ExprNode,
}

impl TypecastExpr {
    pub fn new(type_name: SystemType, value: ExprNode) -> Self {
        Self { type_name, value }
    }

    pub fn type_name(&self) -> &SystemType {
        &self.type_name
    }

    pub fn value(&self) -> &ExprNode {
        &self.value
    }
}

use crate::parser::ast_node::{ExprNode, SystemType};
