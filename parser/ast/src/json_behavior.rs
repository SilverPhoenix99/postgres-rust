#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonBehavior {
    Null,
    Error,
    True,
    False,
    Unknown,
    EmptyArray,
    EmptyObject,
    Default(ExprNode),
}

use crate::ExprNode;
