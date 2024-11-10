#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QualifiedOperator(pub Vec<CowStr>, pub Operator);

impl From<Operator> for QualifiedOperator {
    fn from(value: Operator) -> Self {
        Self(vec![], value)
    }
}

use crate::parser::{
    ast_node::Operator,
    CowStr
};
