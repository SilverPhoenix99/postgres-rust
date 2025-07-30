#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubstringFunc {
    ExplicitCall(Option<Vec<NamedValue>>),
    SqlSyntax(ExprNode, ExprNode, Option<ExprNode>),
}

impl Default for SubstringFunc {
    fn default() -> Self {
        SubstringFunc::ExplicitCall(None)
    }
}

use crate::ExprNode;
use crate::NamedValue;
