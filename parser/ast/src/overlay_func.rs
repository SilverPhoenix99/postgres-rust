#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlayFunc {
    SqlSyntax(OverlaySqlArgs),
    ExplicitCall(Option<Vec<NamedValue>>),
}

impl Default for OverlayFunc {
    fn default() -> Self {
        OverlayFunc::ExplicitCall(None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlaySqlArgs {
    arg: ExprNode,
    placing: ExprNode,
    from: ExprNode,
    for_expr: Option<ExprNode>,
}

impl OverlaySqlArgs {
    pub fn new(arg: ExprNode, placing: ExprNode, from: ExprNode) -> Self {
        Self {
            arg,
            placing,
            from,
            for_expr: None
        }
    }

    pub fn arg(&self) -> &ExprNode {
        &self.arg
    }

    pub fn placing(&self) -> &ExprNode {
        &self.placing
    }

    pub fn from(&self) -> &ExprNode {
        &self.from
    }

    pub fn with_for_expr(mut self, for_expr: ExprNode) -> Self {
        self.for_expr = Some(for_expr);
        self
    }

    pub fn set_for_expr(&mut self, for_expr: Option<ExprNode>) -> &mut Self {
        self.for_expr = for_expr;
        self
    }

    pub fn for_expr(&self) -> Option<&ExprNode> {
        self.for_expr.as_ref()
    }
}

use crate::ExprNode;
use crate::NamedValue;
