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
    pub fn new(arg: ExprNode, placing: ExprNode, from: ExprNode, for_expr: Option<ExprNode>) -> Self {
        Self { arg, placing, from, for_expr }
    }
}

use crate::ExprNode;
use crate::NamedValue;
