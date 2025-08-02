#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmltableColumnOption {
    Null,
    NotNull,
    Default(ExprNode),
    Path(ExprNode),
}

use crate::ExprNode;
