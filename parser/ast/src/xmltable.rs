#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmltableColumnOption {
    Null,
    NotNull,
    Default(ExprNode),
    Path(ExprNode),
    Generic {
        option: Box<str>,
        value: ExprNode
    },
}

use crate::ExprNode;
