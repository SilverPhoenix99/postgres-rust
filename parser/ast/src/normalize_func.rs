#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NormalizeFunc {
    expr: ExprNode,
    normal_form: Option<UnicodeNormalForm>,
}

impl NormalizeFunc {
    pub fn new(expr: ExprNode, normal_form: Option<UnicodeNormalForm>) -> Self {
        Self { expr, normal_form }
    }

    pub fn expr(&self) -> &ExprNode {
        &self.expr
    }

    pub fn normal_form(&self) -> Option<UnicodeNormalForm> {
        self.normal_form
    }
}

use crate::ExprNode;
use crate::UnicodeNormalForm;
