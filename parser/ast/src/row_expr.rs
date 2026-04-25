#[derive(Debug, Clone, Eq, PartialEq, Into)]
pub struct RowExpr {
    args: Option<Vec<ExprNode>>,
    implicit: bool
}

impl RowExpr {

    pub fn implicit(args: Vec<ExprNode>) -> Self {
        Self {
            args: Some(args),
            implicit: true
        }
    }
    
    pub fn explicit(args: Option<Vec<ExprNode>>) -> Self {
        Self {
            args,
            implicit: false
        }
    }

    pub fn args(&self) -> Option<&[ExprNode]> {
        self.args.as_deref()
    }

    pub fn is_implicit(&self) -> bool {
        self.implicit
    }

    pub fn is_explicit(&self) -> bool {
        !self.implicit
    }
}

use crate::ExprNode;
use derive_more::Into;
